use std::collections::HashMap;

use anyhow::Result;

use timer::Timer;
use levels::Chapter;
use saves::AreaModeStats;

mod timer;
mod levels;
mod saves;
mod watch;

fn main() -> Result<()> {

    let timer = Timer::new()?;
    timer.run()?;
    Ok(())
}

pub fn print_times(save: &HashMap<Chapter, AreaModeStats>) {
    for key in levels::ANY_PERCENT_ROUTE {
        println!("{:?}: {:?}", key.to_string(), save[&key]);
    }
}
