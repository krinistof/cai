use common::{Args, ClientExt};
use anyhow::{Context, Result};
use dotenv::dotenv;
use openai_api_rs::v1::api::Client;
use clap::Parser;
use std::env;

mod common;
mod execute;

fn main() -> Result<()> {
    dotenv()?;

    let client = Client::new(
        env::var("OPENAI_API_KEY").with_context(|| "No OPENAI_API_KEY env var given!")?,
    );

    client.eval(Args::parse())
}
