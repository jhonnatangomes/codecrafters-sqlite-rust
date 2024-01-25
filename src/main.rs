use anyhow::{bail, Result};
use dbinfo::DbInfo;

mod dbinfo;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let db_info = DbInfo::try_from(&args[1])?;
            println!("{db_info}");
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
