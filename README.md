![banner](https://user-images.githubusercontent.com/23558090/198913411-730bd7ff-3d9b-4a5e-831c-55691f97e11a.jpg)


# Automagically-generated commit messages

A CLI tool that generates commit messages from your staged changes, built in Rust and using [OpenAI's Codex](https://openai.com/blog/openai-codex/).

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

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project with by running `cargo build`, and run it with `cargo run`.

## License
This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
