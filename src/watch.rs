use std::{collections::HashMap, path::Path, sync::Mutex};

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
        let last_save: Mutex<Option<HashMap<Chapter, AreaModeStats>>> = Mutex::new(None);

        let watcher = RecommendedWatcher::new(move |res: notify::Result<Event>| {
            futures::executor::block_on(async {
                if let Ok(event) = res {
                    if let EventKind::Modify(kind) = event.kind {
                        if let ModifyKind::Data(_) = kind {
                            let path = event.paths.get(0).expect("could not get path for watcher");
                            match crate::saves::load_save(path.as_ref()) {
                                Ok(data) => {
                                    let already_printed = if let Some(old_data) = last_save.lock().unwrap().as_ref() {
                                        // We assume that a save file with more chapters completed
                                        // is more recent than one with fewer completed
                                        data.len() <= old_data.len()
                                    } else {
                                        false
                                    };
                                    if !already_printed {
                                        if let Err(e) = tx.send(data.clone()).await {
                                            println!("error sending save data from watcher: {:?}", e);
                                        }
                                        *last_save.lock().unwrap() = Some(data);
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
