use std::{path::Path, sync::Mutex, collections::HashMap, time::Duration, fs::File};

use anyhow::{anyhow, Result};
use futures::{StreamExt, select, future::FutureExt};
use crossterm::event::{EventStream, Event, KeyCode};

use crate::{watch::AsyncWatcher, terminal::Terminal, table::{Table, TableCell}, levels::Chapter, saves::TimeMap};

const PB_PATH: &str = "pb.json";
const BEST_SPLITS_PATH: &str = "best_splits.json";

pub struct Timer {
    watcher: AsyncWatcher,
    terminal: Mutex<Terminal>,
    current_save: Option<TimeMap>,
    pb: TimeMap,
    best_splits: TimeMap,
}

impl Timer {
    pub fn new() -> Result<Self> {
        let path_str = shellexpand::full("$XDG_DATA_HOME/Celeste/Saves/2.celeste")?;
        let path_str = path_str.as_ref();
        let path = Path::new(path_str);
        let current_save = crate::saves::load_save(path).ok();

        let watcher = AsyncWatcher::new(path)?;

        let terminal = Mutex::new(Terminal::new()?);

        let pb_reader = File::open(PB_PATH);
        let pb = if let Ok(reader) = pb_reader {
            serde_json::from_reader(reader).unwrap_or_else(|_| {
                terminal.lock().unwrap().write_error("could not deserialize pb from file. initializing empty pb").unwrap();
                HashMap::new()
            })
        } else {
            terminal.lock().unwrap().write_status_default("could not open pb file. initializing empty pb").unwrap();
            HashMap::new()
        };

        let best_splits_reader = File::open(BEST_SPLITS_PATH);
        let best_splits = if let Ok(reader) = best_splits_reader {
            serde_json::from_reader(reader).unwrap_or_else(|_| {
                terminal.lock().unwrap().write_error("could not deserialize best splits from file. initializing empty best splits").unwrap();
                HashMap::new()
            })
        } else {
            terminal.lock().unwrap().write_status_default("could not open best splits file. initializing empty best splits").unwrap();
            HashMap::new()
        };

        Ok(Self {
            watcher,
            terminal,
            current_save,
            pb,
            best_splits,
        })
    }

    pub fn run(mut self) -> Result<()> {
        futures::executor::block_on(async {
            let mut key_reader = EventStream::new();
            if let Err(e) = self.on_save_update(&crate::levels::ANY_PERCENT_ROUTE) {
                self.terminal.lock().unwrap().write_error(format!("an error occurred: {:?}", e).as_str()).unwrap();
            }
            loop {
                let rx = &mut self.watcher.watcher_rx;
                let mut recv = rx.next().fuse();
                let mut key_event = key_reader.next().fuse();
                select! {
                    data = recv => {
                        if let Some(data) = data {
                            self.current_save = Some(data);
                            if let Err(e) = self.on_save_update(&crate::levels::ANY_PERCENT_ROUTE) {
                                self.terminal.lock().unwrap().write_error(format!("an error occurred: {:?}", e).as_str()).unwrap();
                            }
                        } else {
                            break;
                        }
                    },
                    maybe_event = key_event => {
                        match maybe_event {
                            Some(Ok(event)) => {
                                if let Event::Key(key) = event {
                                    if self.handle_key(key.code) {
                                        break;
                                    }
                                }
                            },
                            Some(Err(e)) => {self.terminal.lock().unwrap().write_error(format!("error while getting key: {:?}", e).as_str()).unwrap();},
                            None => break,
                        }
                    }
                }
            }
            self.save_data().unwrap();
        });
        Ok(())
    }

    fn on_save_update(&mut self, route: &[Chapter]) -> Result<()> {

        self.print_times(route)?;

        let data = self.current_save.as_ref().ok_or(anyhow!("no current save!"))?;
        let TimeTotals { total_time, pb_total, .. } = self.get_time_totals(route);
        let mut term = self.terminal.lock().unwrap();

        if data.len() >= route.len() {
            if total_time <= pb_total {
                term.write_status("new personal best! congratulations!", crossterm::style::Color::Green)?;
                self.pb = data.clone();
            }
            for chapter in route {
                let time = data.get(chapter).ok_or(anyhow!("could not get time for chapter {}", chapter.to_string()))?;
                if let Some(best_split) = self.best_splits.get(chapter) {
                    if time < best_split {
                        self.best_splits.insert(chapter.clone(), time.clone());
                    }
                }
            }
        }
        self.save_data()

    }

    fn print_times(&mut self, route: &[Chapter]) -> Result<()> {
        let mut term = self.terminal.lock().unwrap();
        let mut table = Table::from_default_header();

        let TimeTotals { total_time, pb_total_running, .. } = self.get_time_totals(route);
        let data = self.current_save.as_ref().ok_or(anyhow!("no current save!"))?;

        for chapter in route {
            if let Some(run_time) = data.get(chapter) {
                let pb_time = self.pb.get(chapter);
                let best_split_time = self.best_splits.get(chapter);

                let chapter_cell = TableCell::new_default(chapter.to_string().as_str());
                let split_time_cell = TableCell::from_duration(run_time);
                let diff_cell = if let Some(pb_time) = pb_time {
                    TableCell::from_diff(pb_time, run_time, run_time < best_split_time.unwrap_or(&Duration::MAX))
                } else {
                    TableCell::new_default("-")
                };
                table.push_row(vec![chapter_cell, split_time_cell, diff_cell]);
            }
        }
        table.push_row(vec![TableCell::new_default("Total"), TableCell::from_duration(&total_time), TableCell::from_diff(&pb_total_running, &total_time, false)]);
        term.write_table(&table)
    }

    fn get_time_totals(&self, route: &[Chapter]) -> TimeTotals {
        let mut total_time = Duration::ZERO;

        let mut pb_total = Duration::ZERO;
        let mut pb_total_running = Duration::ZERO;

        for chapter in route {
            if let Some(data) = self.current_save.as_ref() {
                if let Some(time) = data.get(chapter) {
                    total_time += *time;
                    pb_total_running += *self.pb.get(chapter).unwrap_or(&Duration::ZERO);
                }
            }
            pb_total += *self.pb.get(chapter).unwrap_or(&Duration::ZERO);
        }

        TimeTotals { total_time, pb_total, pb_total_running }
    }

    fn save_data(&self) -> Result<()> {
        let pb_writer = File::create(PB_PATH)?;
        serde_json::to_writer(pb_writer, &self.pb)?;
        let best_splits_writer = File::create(BEST_SPLITS_PATH)?;
        serde_json::to_writer(best_splits_writer, &self.best_splits)?;

        Ok(())
    }

    fn handle_key(&mut self, keycode: KeyCode) -> bool {
        match keycode {
            KeyCode::Char('q') => return true,
            _ => (),
        }
        false
    }
}

struct TimeTotals {
    total_time: Duration,
    pb_total: Duration,
    pb_total_running: Duration,
}
