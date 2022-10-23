use anyhow::Result;

use timer::Timer;

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
