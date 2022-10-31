![banner](https://user-images.githubusercontent.com/23558090/198913411-730bd7ff-3d9b-4a5e-831c-55691f97e11a.jpg)

# Automagically-generated commit messages

A CLI tool that generates commit messages from your staged changes, built in Rust and using [OpenAI's Codex](https://openai.com/blog/openai-codex/).

## Installation

You can install `auto-commit` by running the following command in your terminal.

```
curl -fsSL https://raw.githubusercontent.com/m1guelpf/auto-commit/main/install.sh | sh -
```

Or, if you're an arch user, you can download it from the [AUR](https://aur.archlinux.org/) using

```sh
yay -S auto-commit
```

You may need to close and reopen your terminal after installation. Alternatively, you can download the binary corresponding to your OS from the [latest release](https://github.com/m1guelpf/auto-commit/releases/latest).

## Usage

`auto-commit` uses [OpenAI's Codex](https://openai.com/blog/openai-codex/), which is currently in private beta. To use it, you'll first need to to [request access to Codex](http://beta.openai.com/codex-waitlist). Once you get access, grab an API key from [your dashboard](https://beta.openai.com/), and save it to `OPENAI_API_KEY` as follows (you can also save it in your bash/zsh profile for presistance between sessions).

> **Note** If you don't wanna wait to get access to Codex (or don't wanna use your API key), check the [Cloud](#cloud-service) section below.

```bash
export OPENAI_API_KEY='sk-XXXXXXXX'
```

Once you have configured your environment, stage some changes by running, for example, `git add .`, and then run `auto-commit`.

Of course, `auto-commit` also includes some options, for editing the message before commiting, or just printing the message to the terminal.

```sh
$ auto-commit --help
Automagically generate commit messages.

Usage: auto-commit [OPTIONS]

Options:
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
      --dry-run     Output the generated message, but don't create a commit.
  -r, --review      Edit the generated commit message before committing.
  -h, --help        Print help information
  -V, --version     Print version information
```

### Cloud Service

Since Codex isn't available to everyone yet, I'm considering setting up an API service that you can use instead (which the CLI would also support). If this is something you'd be interested in, you can [preorder it here](https://m1guelpf.me/auto-commit-cloud).

> If enough people preorder the cloud service, I'll have it shipped by the end of the week. Otherwise, you'll get a full refund.

## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project with by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
