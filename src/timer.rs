use std::{path::Path, sync::Mutex, collections::HashMap, time::Duration, fs::File};

use anyhow::Result;
use futures::{StreamExt, select, future::FutureExt};
use crossterm::event::{EventStream, Event, KeyCode};

use crate::{watch::AsyncWatcher, terminal::Terminal, table::Table, levels::Chapter, saves::TimeMap};

const PB_PATH: &str = "pb.json";
const BEST_SPLITS_PATH: &str = "best_splits.json";

pub struct Timer {
    watcher: AsyncWatcher,
    terminal: Mutex<Terminal>,
    pb: HashMap<Chapter, Duration>,
    best_splits: HashMap<Chapter, Duration>,
}

impl Timer {
    pub fn new() -> Result<Self> {
        let path_str = shellexpand::full("$XDG_DATA_HOME/Celeste/Saves/2.celeste")?;
        let path_str = path_str.as_ref();
        let path = Path::new(path_str);

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
            terminal.lock().unwrap().write_error("could not deserialize best splits from file. initializing empty best splits").unwrap();
            serde_json::from_reader(reader).unwrap_or_else(|_| HashMap::new())
        } else {
            terminal.lock().unwrap().write_status_default("could not open best splits file. initializing empty best splits").unwrap();
            HashMap::new()
        };

        Ok(Self {
            watcher,
            terminal,
            pb,
            best_splits,
        })
    }

    pub fn run(mut self) -> Result<()> {
        futures::executor::block_on(async {
            let mut key_reader = EventStream::new();
            loop {
                let rx = &mut self.watcher.watcher_rx;
                let mut recv = rx.next().fuse();
                let mut key_event = key_reader.next().fuse();
                select! {
                    data = recv => {
                        if let Some(data) = data {
                            if let Err(e) = self.on_save_update(&data, &crate::levels::ANY_PERCENT_ROUTE) {
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

    fn on_save_update(&mut self, data: &TimeMap, route: &[Chapter]) -> Result<()> {
        let mut term = self.terminal.lock().unwrap();
        term.write_status_default("got save update!")?;
        term.write_table(&Table::from_times(&data, route))?;

        let mut time = Duration::ZERO;

        let mut pb_total = Duration::ZERO;
        let mut pb_total_running = Duration::ZERO;
        let zero = &Duration::ZERO;

        for chapter in route {
            if let Some(run_time) = data.get(chapter) {
                time += *run_time;
                pb_total_running += *self.pb.get(chapter).unwrap_or(zero);
            }
                pb_total += *self.pb.get(chapter).unwrap_or(zero);
        }

        Ok(())
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
