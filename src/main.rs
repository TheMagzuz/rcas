use anyhow::Result;

mod levels;
mod saves;

fn main() -> Result<()> {
    let path = "/home/markus/.local/share/Celeste/Saves/2.celeste";
    let save = saves::load_save(path)?;
    for key in levels::ANY_PERCENT_ROUTE {
        println!("{:?}: {:?}", key.to_string(), save[&key]);
    }
    Ok(())
}
