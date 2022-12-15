use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::{error, info};
use question::{Answer, Question};
use std::{
    fs,
    fs::File,
    io::Write,
    process::{Command, Stdio},
    str,
};

use rand::seq::SliceRandom;
use spinners::{Spinner, Spinners};

#[derive(Parser)]
#[command(version)]
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

    #[arg(short, long, help = "Don't ask for confirmation before committing.")]
    force: bool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let api_token = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        error!("Please set the OPENAI_API_KEY environment variable.");
        std::process::exit(1);
    });

    let git_staged_cmd = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .output()
        .expect("Couldn't find diff.")
        .stdout;

    let git_staged_cmd = str::from_utf8(&git_staged_cmd).unwrap();

    if git_staged_cmd.len() == 0 {
        error!("There are no staged files to commit.\nTry running `git add` to stage some files.");
    }

    let is_repo = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect("Failed to check if this is a git repository.")
        .stdout;

    if str::from_utf8(&is_repo).unwrap().trim() != "true" {
        error!("It looks like you are not in a git repository.\nPlease run this command from the root of a git repository, or initialize one using `git init`.");
        std::process::exit(1);
    }

    let client = openai_api::Client::new(&api_token);

    let output = Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .output()
        .expect("Couldn't find diff.")
        .stdout;
    let output = str::from_utf8(&output).unwrap();

    if !cli.dry_run {
        info!("Loading Data...");
    }

    let prompt_args = openai_api::api::CompletionArgs::builder()
        .prompt(format!(
            "git diff HEAD\\^!\n{}\n\n# Write a commit message describing the changes and the reasoning behind them\ngit commit -F- <<EOF",
            output
        ))
        .engine("code-davinci-002")
        .temperature(0.0)
        .max_tokens(2000)
        .stop(vec!["EOF".into()]);

    let sp: Option<Spinner> = if !cli.dry_run && cli.verbose.is_silent() {
        let vs = [
            Spinners::Earth,
            Spinners::Aesthetic,
            Spinners::Hearts,
            Spinners::BoxBounce,
            Spinners::BoxBounce2,
            Spinners::BouncingBar,
            Spinners::Christmas,
            Spinners::Clock,
            Spinners::FingerDance,
            Spinners::FistBump,
            Spinners::Flip,
            Spinners::Layer,
            Spinners::Line,
            Spinners::Material,
            Spinners::Mindblown,
            Spinners::Monkey,
            Spinners::Noise,
            Spinners::Point,
            Spinners::Pong,
            Spinners::Runner,
            Spinners::SoccerHeader,
            Spinners::Speaker,
            Spinners::SquareCorners,
            Spinners::Triangle,
        ];

        let spinner = vs.choose(&mut rand::thread_rng()).unwrap().clone();

        Some(Spinner::new(spinner, "Analyzing Codebase...".into()))
    } else {
        None
    };

    let completion = client
        .complete_prompt(prompt_args.build().unwrap())
        .await
        .expect("Couldn't complete prompt.");

    if sp.is_some() {
        sp.unwrap().stop_with_message("Finished Analyzing!".into());
    }

    let mut commit_msg = completion.choices[0].text.to_owned();

    if cli.dry_run {
        info!("{}", commit_msg);
        return Ok(());
    } else {
        info!(
            "Proposed Commit:\n------------------------------\n{}\n------------------------------",
            commit_msg
        );

        if !cli.force {
            let answer = Question::new("Do you want to continue? [Y]es [e]dit [n]o")
                .accept("yes")
                .accept("y")
                .accept("edit")
                .accept("e")
                .accept("no")
                .accept("n")
                .default(Answer::RESPONSE(String::from("yes")))
                .until_acceptable()
                .ask()
                .expect("Couldn't ask question.");

            if let Answer::RESPONSE(input) = answer {
                let input_lower = input.to_lowercase();
                match input_lower.as_str() {
                    "no" | "n" => {
                        error!("Commit aborted by user.");
                        std::process::exit(1);
                    }

                    "edit" | "e" => {
                        let editor = match std::env::var("EDITOR") {
                            Ok(val) => val,
                            Err(_) => "vi".to_string(),
                        };
                        let mut commit_msg_file =
                            File::create(".auto_commit_msg").expect("Couldn't create temp file");
                        commit_msg_file
                            .write_all(commit_msg.as_bytes())
                            .expect("Couldn't write commit msg into temp file");

                        Command::new("/usr/bin/sh")
                            .arg("-c")
                            .arg(format!("{} .auto_commit_msg", editor))
                            .spawn()
                            .expect("Error: Failed to run editor")
                            .wait()
                            .expect("Error: Editor returned a non-zero status");

                        commit_msg = fs::read_to_string(".auto_commit_msg")
                            .expect("Cloudn't read commit message from file");
                        fs::remove_file(".auto_commit_msg").expect("Failed to delete temp file");

                        if commit_msg.len() == 0 {
                            error!("Commit aborted by user.");
                            std::process::exit(1);
                        }

                        info!("Using new commit message:\n{}", commit_msg);
                    }
                    _ => {}
                }
            }

            info!("Committing Message...");
        }
    }

    let mut ps_commit = Command::new("git")
        .arg("commit")
        .args(if cli.review { vec!["-e"] } else { vec![] })
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

    let commit_output = ps_commit
        .wait_with_output()
        .expect("There was an error when creating the commit.");

    info!("{}", str::from_utf8(&commit_output.stdout).unwrap());

    Ok(())
}
