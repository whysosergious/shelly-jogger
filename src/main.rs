#![allow(unused_imports)]

use std::process::Stdio;
use std::{env, process::exit};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use ansi_term::Colour::*;

#[tokio::main]
async fn main() {


    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", Black.on(Yellow).bold().paint("Please provide a command to run"));
        exit(0);
    }

    // Remove the program name e.g., the main cmd binary `run_with_callback`
    args.remove(0);

    let mut commands: Vec<Vec<String>> = args
        .split(|arg| arg == "--")
        .map(|arg| arg.to_vec())
        .collect();

    // Spawn a new task for running the commands
    tokio::spawn(async move {
        if commands.is_empty() {
            eprintln!("No command provided to run");
            return;
        }

        let cmd = commands[0].remove(0);
        let args = commands[0].drain(..).collect::<Vec<String>>();
        commands.remove(0); // remove the now empty vector

        println!("»»» {}", cmd);

        let mut child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        let stdout_reader = BufReader::new(stdout).lines();
        let stderr_reader = BufReader::new(stderr).lines();

        tokio::spawn(async move {
            let mut stdout_reader = stdout_reader;
            while let Some(line) = stdout_reader.next_line().await.unwrap() {
                println!("{}", Blue.paint(line));
            }
        });

        tokio::spawn(async move {
            let mut stderr_reader = stderr_reader;
            while let Some(line) = stderr_reader.next_line().await.unwrap() {
                println!("{}", Blue.paint(line));
            }
        });

        let status = child.wait().await.expect("Failed to wait on child process");

        if status.success() {
            println!("{:?}", White.paint("ᓚᘏᗢ").to_string());
        } else {
            eprintln!("process failed with status: {}", status);
        }
    })
    .await
    .unwrap();
}
