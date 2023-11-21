use std::{env, fs::File, io::{Read, Write}};
use openai_api_rs::v1::api::Client;
use dotenv::dotenv;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().into());

    let args: Vec<_> = env::args().collect();
    let Some(path) = args.get(1) else {
        return Err("Not path given!".into());
    };

    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut req = toml::from_str(&contents)?;

    client.continue_chat(&mut req)?;

    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(path)?;

    file.write_all(toml::to_string(&req)?.as_bytes())?;

    Ok(())
}
