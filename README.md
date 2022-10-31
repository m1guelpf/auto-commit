![auto-commit Banner](https://user-images.githubusercontent.com/23558090/198913031-a4444a04-3151-42e9-9db6-7605f14be955.jpg)

# Automatically-generated commit messages


> Legible commit messages without the bother!

A CLI tool that generates commit messages from your staged changes, using [OpenAI's Codex](https://openai.com/blog/openai-codex/).

## Installation

You can install `auto-commit` by running the following command in your terminal.

```
curl -fsSL https://raw.githubusercontent.com/m1guelpf/auto-commit/main/install.sh | sh -
```

You may need to close and reopen your terminal after installation. Alternatively, you can download the binary corresponding to your OS from the [latest release](https://github.com/m1guelpf/auto-commit/releases/latest).

## Usage

To automagically generate commit messages, simply add your files using for example

```sh
git add .
```

And then run

```sh
auto-commit
```

If you would rather edit the commit message you can run

```sh
auto-commit -r
```

And if you only want to see the result and not commit it immediately you can dry-run using

```sh
auto-commit --dry-run
```

### API Keys (Requires [Open AI](https://beta.openai.com/))

In order for `auto-commit` to work you need to get an [OPENAI API Key](https://beta.openai.com/), and have [access to Codex](http://beta.openai.com/codex-waitlist). Once you have those make sure to have the environment variable set as follows.

`OPENAI_API_KEY='sk-XXXXXXXX'`

## Develop

To build this package, make sure you have the latest version of rust installed (using for ex [rustup](https://rustup.rs/)) and then simply run the following command.

```sh
cargo build
```

## License
This project is open-sourced software licensed under the MIT license. See [the License file](https://github.com/m1guelpf/auto-commit/blob/main/LICENSE) for more information.
