use std::{collections::HashMap, path::Path};

use anyhow::Result;
use futures::{channel::mpsc::{channel, Receiver}, SinkExt, StreamExt};
use notify::{event::{Event, ModifyKind}, RecommendedWatcher, Watcher, RecursiveMode, EventKind};

use crate::{levels::Chapter, saves::AreaModeStats};

pub struct AsyncWatcher {
    // This needs to be on the struct, since it will otherwise go out of scope, and therefore stop
    // watching
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    watcher_rx: Receiver<HashMap<Chapter, AreaModeStats>>,
}

impl AsyncWatcher {
    pub fn new(path: &Path) -> Result<Self> {
        let (mut watcher, watcher_rx) = Self::create_watcher()?;

        watcher.watch(path, RecursiveMode::NonRecursive)?;

        Ok(Self { watcher, watcher_rx })
    }

    pub async fn watch(&mut self) {
        while let Some(data) = self.watcher_rx.next().await {
            crate::print_times(&data);
        }
    }


    fn create_watcher() -> Result<(RecommendedWatcher, Receiver<HashMap<Chapter, AreaModeStats>>)> {
        let (mut tx, rx) = channel(1);

        let watcher = RecommendedWatcher::new(move |res: notify::Result<Event>| {
            futures::executor::block_on(async {
                if let Ok(event) = res {
                    if let EventKind::Modify(kind) = event.kind {
                        if let ModifyKind::Data(_) = kind {
                            let path = event.paths.get(0).expect("could not get path for watcher");
                            match crate::saves::load_save(path.as_ref()) {
                                Ok(data) => {
                                    if let Err(e) = tx.send(data).await {
                                        println!("error sending save data from watcher: {:?}", e);
                                    }
                                },
                                Err(e) => println!("unable to load save data: {:?}", e),
                            }
                        }
                    }
                }
            })
        }, notify::Config::default())?;

        Ok((watcher, rx))
    }
}
