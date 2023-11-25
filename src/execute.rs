use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use openai_api_rs::v1::chat_completion::{ChatCompletionMessage, ChatCompletionRequest};
use regex::Regex;
use std::process::Command;

pub fn get_command(req: &ChatCompletionRequest) -> Result<Option<String>> {
    const COMMAND_REGEX: &str = r"(?s)```sh\n(?<command>.*)\n```";
    lazy_static! {
        static ref RE: Regex = Regex::new(COMMAND_REGEX).unwrap();
    };

    let ChatCompletionMessage { content, .. } = req
        .messages
        .last()
        .context("Cannot find content in the last message!")?;

    let Some(capture) = RE.captures(content) else {
        return Ok(None);
    };

    let command = capture.name("command").unwrap().as_str();

    Ok(Some(command.into()))
}

pub fn run(cmd: &str) -> Result<String> {
    let output = Command::new("sh").arg("-c").arg(cmd).output()?;

    if !output.status.success() {
        bail!(
            "command `{cmd}` failed with following stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into())
}
