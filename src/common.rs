use crate::execute::{run, get_command};
use anyhow::Result;
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{ChatCompletionMessage, ChatCompletionRequest, MessageRole},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub fn read_file(input_path: &PathBuf) -> Result<String> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_to_file(input_path: &PathBuf, contents: &str) -> Result<()> {
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(input_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn process_request(
    client: &Client,
    req: &mut ChatCompletionRequest,
    execute: bool,
) -> Result<()> {
    if execute {
        if let Some(command) = get_command(req)? {
            let output = run(&command)?;
            req.messages.push(ChatCompletionMessage {
                role: MessageRole::user,
                content: format!("output of command: {output}"),
                name: None,
                function_call: None,
            });
        }
    } else {
        client.continue_chat(req)?;
    }
    Ok(())
}
