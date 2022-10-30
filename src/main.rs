use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::{
    io::Write,
    process::{Command, Stdio},
    str,
};

#[derive(Parser)]
#[command(name = "Auto Commit")]
#[command(author = "Miguel Piedrafita <soy@miguelpiedrafita.com>")]
#[command(about = "Automagically generate commit messages.", long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[arg(
        long = "dry-run",
        help = "Output the generated message, but don't create a commit."
    )]
    dry_run: bool,

    #[arg(
        short,
        long,
        help = "Edit the generated commit message before committing."
    )]
    review: bool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let api_token = std::env::var("OPENAI_API_KEY").expect("No API key specified.");
    let client = openai_api::Client::new(&api_token);

    let output = Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .output()
        .expect("Couldn't find diff.")
        .stdout;
    let output = str::from_utf8(&output).unwrap();

    let prompt_args = openai_api::api::CompletionArgs::builder()
        .prompt(format!(
            "git diff HEAD\\^!\n{}\n\n# Write a commit message describing the changes and the reasoning behind them\ngit commit -F- <<EOF",
            output
        ))
        .engine("code-davinci-002")
        .temperature(0.0)
        .max_tokens(2000)
        .stop(vec!["EOF".into()]);

    let completion = client
        .complete_prompt(prompt_args.build().unwrap())
        .await
        .expect("Couldn't complete prompt.");

    let commit_msg = completion.choices[0].text.to_owned();

    if cli.dry_run {
        println!("{}", commit_msg);
        return Ok(());
    }

    let mut ps_commit = Command::new("git")
        .arg("commit")
        .arg("-F")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = ps_commit.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(commit_msg.as_bytes())
            .expect("Failed to write to stdin");
    });

    let mut commit_output = ps_commit
        .wait_with_output()
        .expect("There was an error when creating the commit.");

    if cli.review {
        commit_output = Command::new("git")
            .arg("commit")
            .arg("--amend")
            .spawn()
            .expect("Failed to open editor.")
            .wait_with_output()
            .expect("Failed to edit commit.");
    }

    println!("{}", str::from_utf8(&commit_output.stdout).unwrap());

    Ok(())
}
