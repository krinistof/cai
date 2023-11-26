use crate::execute::{get_command, run};
use anyhow::Result;
use clap::Parser;
use lazy_static::lazy_static;
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{ChatCompletionMessage, MessageRole},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(about, arg_required_else_help(true))]
#[group(id = "command", args(vec![ "input_path", "example" ]))]
pub struct Args {
    #[arg(short = 'x', long)]
    pub execute: bool,
    #[arg(short = 'e', long, group = "input")]
    pub example: bool,
    #[arg(group = "input")]
    pub input_path: Option<PathBuf>,
}

impl Args {}

pub trait ClientExt {
    fn eval(self, args: Args) -> Result<()>;
}

impl ClientExt for Client {
    fn eval(self, args: Args) -> Result<()> {
        match args {
            Args { example, .. } if example => {
                lazy_static! {
                    static ref FILE_PATH: PathBuf = PathBuf::from("new.toml");
                };
                let path = FILE_PATH.display();
                println!("Example loaded into {path}!");
                const EXAMPLE_CONTENT: &str = include_str!("../examples/life.toml");

                write_to_file(&FILE_PATH, EXAMPLE_CONTENT)
            }
            Args {
                input_path: Some(ref input_path),
                execute,
                ..
            } => {
                let contents = read_file(input_path)?;
                let mut chat = toml::from_str(contents.as_str())?;

                if execute {
                    if let Some(command) = get_command(&chat)? {
                        let output = run(&command)?;
                        chat.messages.push(ChatCompletionMessage {
                            role: MessageRole::user,
                            content: format!("output of command:\n```\n{output}\n```\n"),
                            name: None,
                            function_call: None,
                        });
                    }
                } else {
                    // TODO move continue_chat to ClientExt trait from dependency
                    self.continue_chat(&mut chat)?;
                }

                write_to_file(input_path, &toml::to_string(&chat)?)
            }
            _ => Ok(println!("`{args:?}` didn't match!")),
        }
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
