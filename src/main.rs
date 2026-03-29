use anyhow::Result;
use crate::config::load_config;

mod config;

fn main() -> Result<()> {
    let config = load_config()?;

    println!("{config:#?}");
    Ok(())
}
