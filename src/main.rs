use anyhow::{Context, Result};
use cai::common::{Args, ClientExt};
use clap::Parser;
use dotenv::dotenv;
use openai_api_rs::v1::api::Client;
use std::env;

fn main() -> Result<()> {
    dotenv()?;

    let client = Client::new(
        env::var("OPENAI_API_KEY").with_context(|| "No OPENAI_API_KEY env var given!")?,
    );

    client.eval(Args::parse())
}
