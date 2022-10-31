# Automatically-generated commit messages

> Legible commit messages without the bother!

A CLI tool that generates commit messages from your staged changes, using [OpenAI's Codex](https://openai.com/blog/openai-codex/).

## Installing

Run the following in your terminal, then follow the onscreen instructions.

```
curl -fsSL https://raw.githubusercontent.com/m1guelpf/auto-commit/main/install.sh | sh -
```

Or if you want to have the binaries, check out the [releases section](https://github.com/m1guelpf/auto-commit/releases).

### API Keys

In order for `auto-commit` to work you need to get an [OPENAI API Key](https://beta.openai.com/), and have [access to Codex](http://beta.openai.com/codex-waitlist). Once you have those make sure to have the environment variable set as follows.

`OPENAI_API_KEY='sk-XXXXXXXX'`

## Develop

To build this package, make sure you have the latest version of rust installed (using for ex [rustup](https://rustup.rs/)) and then simply run the following command.

```
cargo build
```

## License
This project is open-sourced software licensed under the MIT license. See [the License file](https://github.com/m1guelpf/auto-commit/blob/main/LICENSE) for more information.
