use std::{path::Path, sync::Mutex};

use anyhow::Result;
use futures::StreamExt;

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
            while let Some(data) = rx.next().await {
                let mut term = self.terminal.lock().unwrap();
                term.write_table(&Table::from_times(data, &crate::levels::ANY_PERCENT_ROUTE)).unwrap();
            }
        });
        Ok(())
    }
}
