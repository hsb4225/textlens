mod cli;
mod analyzer;

use anyhow::Result;

fn main() -> Result<()> {
    let config = cli::parse_args();
    analyzer::run(config)?;
    Ok(())
}

