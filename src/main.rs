mod analyzer;
mod cli;

use crate::cli::Config;
use crate::analyzer::{analyze, print_result};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    // top 옵션이 None이면 기본 10
    let top_n = config.top.unwrap_or(10);

    let result = analyze(&config.filepath, top_n)?;
    print_result(result, top_n, config.chars, config.json)
}


