use std::collections::HashMap;

use anyhow::Result;

use timer::Timer;
use levels::Chapter;
use saves::AreaModeStats;

mod timer;
mod levels;
mod saves;
mod watch;
mod terminal;
mod table;

fn main() -> Result<()> {
    let timer = Timer::new()?;
    timer.run()?;
    Ok(())
}
