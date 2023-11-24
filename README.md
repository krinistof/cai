# cai - Context AI

Would you like to level up your gaslighting game? Fake chat history to the OpenAI API today!

## Usage

Make sure to [get your API key](https://help.openai.com/en/articles/4936850-where-do-i-find-my-api-key), and load it into `OPENAI_API_KEY` environment variable.
Note: `.env` variable loading is also supported.

To continue your chat, execute:
```sh
cai <path-to-conversation>
```

See `life.toml` in the `examples` directory.
```sh
$ cat life.toml
model = "gpt-4"

[[messages]]
role = "user"
content = "What is the meaning of life?"

$ cai life.toml
# After several seconds...

$ cat life.toml
model = "gpt-4"

[[messages]]
role = "user"
content = "What is the meaning of life?"

[[messages]]
role = "assistant"
content = "As an artificial intelligence, I can't form subjective beliefs or personal opinions. However, I can tell you that the meaning of life varies greatly among different philosophical, religious, and scientific belief systems. Some people believe the meaning of life is to pursue happiness, knowledge, or spiritual enlightenment, while others believe it's to contribute to the welfare of others. Ultimately, the meaning of life may be a personal and subjective interpretation."
```

## Installation

See [rustup](https://rustup.rs) for installing Rust for your system.
After getting this project's source code, execute:
```sh
git submodule update --init
cargo install --path .
```
This should build `cai`, and place it into `~/.cargo/bin`. Make sure to add this to your `PATH`.
