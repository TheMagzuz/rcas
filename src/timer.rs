use std::{path::Path, sync::Mutex};

use anyhow::Result;
use futures::{StreamExt, select, future::FutureExt};
use crossterm::{event::{EventStream, Event, KeyCode}, style::Color};

use crate::{watch::AsyncWatcher, terminal::Terminal, table::Table};

pub struct Timer {
    watcher: AsyncWatcher,
    terminal: Mutex<Terminal>,
}

impl Timer {
    pub fn new() -> Result<Self> {
        let path_str = shellexpand::full("$XDG_DATA_HOME/Celeste/Saves/2.celeste")?;
        let path_str = path_str.as_ref();
        let path = Path::new(path_str);

        let watcher = AsyncWatcher::new(path)?;

        let terminal = Mutex::new(Terminal::new()?);

        Ok(Self {
            watcher,
            terminal,
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
                            let mut term = self.terminal.lock().unwrap();
                            term.write_table(&Table::from_times(data, &crate::levels::ANY_PERCENT_ROUTE)).unwrap();
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
        });
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
