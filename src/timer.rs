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
            let rx = self.watcher.watch();
            let mut key_reader = EventStream::new();
            loop {
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
                                    if key.code == KeyCode::Char('q') {
                                        break;
                                    }
                                    self.terminal.lock().unwrap().write_status(format!("got keycode {:?}", key.code).as_str(), Color::Reset).unwrap();
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

}
