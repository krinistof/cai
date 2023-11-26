use crate::execute::{get_command, run};
use anyhow::Result;
use clap::Parser;
use lazy_static::lazy_static;
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{ChatCompletionMessage, ChatCompletionRequest, MessageRole},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(about)]
#[group(id = "command", args(vec![ "input_path", "example" ]))]
pub struct Args {
    #[arg(short = 'x', long)]
    pub execute: bool,
    #[arg(short = 'e', long, group = "input")]
    pub example: bool,
    #[arg(require_equals = true, group = "input")]
    pub input_path: Option<PathBuf>,
}

impl Args {}

pub trait ClientExt {
    fn process_request(&self, req: &mut ChatCompletionRequest, args: &Args) -> Result<()>;
    fn eval(self, args: Args) -> Result<()>;
}

impl ClientExt for Client {
    fn eval(self, args: Args) -> Result<()> {
        let Args {
            ref input_path,
            example,
            ..
        } = args;

        if let Some(ref input_path) = input_path {
            let contents = read_file(input_path)?;
            let mut req = toml::from_str(&contents)?;

            self.process_request(&mut req, &args)?;

            write_to_file(input_path, &toml::to_string(&req)?)
        } else {
            if example {
                lazy_static! {
                    static ref FILE_PATH: PathBuf = PathBuf::from("./new.toml");
                };
                const EXAMPLE_CONTENT: &str = include_str!("../examples/life.toml");

                dbg!(&FILE_PATH.as_path());
                write_to_file(&FILE_PATH, EXAMPLE_CONTENT)
            } else {
                Ok(())
            }
        }
    }
    fn process_request(&self, req: &mut ChatCompletionRequest, args: &Args) -> Result<()> {
        let Args {
            execute, example, ..
        } = args;

        // TODO codesmell: match refactor needed
        if *example {
            lazy_static! {
                static ref FILE_PATH: PathBuf = PathBuf::from("new.toml");
            };
            const EXAMPLE_CONTENT: &str = include_str!("../examples/life.toml");

            return write_to_file(&FILE_PATH, EXAMPLE_CONTENT);
        }

        if *execute {
            if let Some(command) = get_command(req)? {
                let output = run(&command)?;
                req.messages.push(ChatCompletionMessage {
                    role: MessageRole::user,
                    content: format!("output of command:\n```\n{output}\n```\n"),
                    name: None,
                    function_call: None,
                });
            }
        } else {
            // TODO move continue_chat to ClientExt trait from dependency
            self.continue_chat(req)?;
        }

        Ok(())
    }
}

pub fn read_file(input_path: &PathBuf) -> Result<String> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_to_file(input_path: &PathBuf, contents: &str) -> Result<()> {
    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(input_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
