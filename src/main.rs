use anyhow::Result;

mod levels;
mod saves;

fn main() -> Result<()> {
    let path = shellexpand::full("$XDG_DATA_HOME/Celeste/Saves/2.celeste")?;
    let path = std::path::Path::new(path.as_ref());
    println!("{:?}", path);
    let save = saves::load_save(&path)?;
    for key in levels::ANY_PERCENT_ROUTE {
        println!("{:?}: {:?}", key.to_string(), save[&key]);
    }
    Ok(())
}
