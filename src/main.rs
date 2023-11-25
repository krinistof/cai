use crate::common::{process_request, read_file, write_to_file};
use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use openai_api_rs::v1::api::Client;
use std::{env, path::PathBuf};

mod common;
mod execute;

#[derive(Parser)]
#[command(about)]
struct Args {
    #[arg(short = 'x', long)]
    execute: bool,
    #[arg(require_equals = true)]
    input_path: PathBuf,
}

fn main() -> Result<()> {
    dotenv()?;

    let client = Client::new(
        env::var("OPENAI_API_KEY").with_context(|| "No OPENAI_API_KEY env var given!")?,
    );

    let Args {
        input_path,
        execute,
    } = Args::parse();

    let contents = read_file(&input_path)?;
    let mut req = toml::from_str(&contents)?;

    process_request(&client, &mut req, execute)?;

    write_to_file(&input_path, &toml::to_string(&req)?)?;

    Ok(())
}
