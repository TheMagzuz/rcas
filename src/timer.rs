use std::path::Path;

use anyhow::Result;

use crate::watch::AsyncWatcher;

pub struct Timer {
    watcher: AsyncWatcher,
}

impl Timer {
    pub fn new() -> Result<Self> {
        let path_str = shellexpand::full("$XDG_DATA_HOME/Celeste/Saves/2.celeste")?;
        let path_str = path_str.as_ref();
        let path = Path::new(path_str);

        let watcher = AsyncWatcher::new(path)?;
        Ok(Self {
            watcher,
        })
    }

    pub fn run(mut self) -> Result<()> {
        futures::executor::block_on(async {
            self.watcher.watch().await;
        });
        Ok(())
    }
}
