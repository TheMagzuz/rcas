use std::{path::Path, collections::HashMap};

use anyhow::Result;

use levels::Chapter;
use saves::AreaModeStats;
use watch::AsyncWatcher;

mod levels;
mod saves;
mod watch;

fn main() -> Result<()> {
    let path_str = shellexpand::full("$XDG_DATA_HOME/Celeste/Saves/2.celeste")?;
    let path_str = path_str.as_ref();
    let path = Path::new(path_str);

    let mut watcher = AsyncWatcher::new(path)?;

    futures::executor::block_on(async {
        watcher.watch().await;
    });

    Ok(())
}

pub fn print_times(save: &HashMap<Chapter, AreaModeStats>) {
    for key in levels::ANY_PERCENT_ROUTE {
        println!("{:?}: {:?}", key.to_string(), save[&key]);
    }
}
