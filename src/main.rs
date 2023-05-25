use std::fs;
use std::io::prelude::*;
use std::process::exit;

use clap::{crate_version, Parser};
use colored::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let opts: Options = Options::parse();

    match opts.subcmd {
        SubCommand::Add(a) => add_hosts_entry(a).await,
        SubCommand::Remove(r) => remove_hosts_entry(r).await,
        SubCommand::List => print_current_entries().await,
    }
}

async fn add_hosts_entry(add: Add) {
    let new_entry = format!("{} {}", add.ip.cyan(), add.hostname.yellow());
    let new_entry_line = format!("{} {}\n", add.ip, add.hostname);

    match fs::read_to_string(HOSTS_PATH) {
        Ok(contents) => {
            if contents.lines().any(|line| line.ends_with(&add.hostname)) {
                eprintln!("{}", format!("Entry already exists: {}", new_entry).red());
                exit(1);
            }
        }
        Err(e) => handle_permission_error(e),
    }

    match fs::OpenOptions::new().append(true).open(HOSTS_PATH) {
        Ok(mut file) => match file.write_all(new_entry_line.as_bytes()) {
            Ok(_) => {
                println!(
                    "{}",
                    format!("Added entry to hosts file: {}", new_entry).green()
                );
            }
            Err(e) => handle_permission_error(e),
        },
        Err(e) => handle_permission_error(e),
    }
}

async fn remove_hosts_entry(remove: Remove) {
    let protected_hostnames = ["localhost", "broadcasthost"];

    if protected_hostnames.contains(&remove.hostname.as_str()) {
        eprintln!(
            "{}",
            format!(
                "Cannot remove protected entry: {}",
                remove.hostname.yellow()
            )
            .red()
        );
        exit(1);
    }

    let mut hosts_file = match File::open(HOSTS_PATH).await {
        Ok(file) => file,
        Err(e) => {
            handle_permission_error(e);
            return;
        }
    };

    let mut contents = String::new();
    match hosts_file.read_to_string(&mut contents).await {
        Ok(_) => {}
        Err(e) => handle_permission_error(e),
    }

    let entry_to_remove = format!("{} {}", remove.ip, remove.hostname);
    let entry_exists = contents.lines().any(|line| line == entry_to_remove);

    if !entry_exists {
        eprintln!(
            "{}",
            format!("Entry does not exist: {}", entry_to_remove.yellow()).red()
        );
        exit(1);
    }

    let entries: Vec<_> = contents
        .lines()
        .filter(|line| !line.ends_with(&remove.hostname))
        .collect();

    match fs::write(HOSTS_PATH, format!("{}\n", entries.join("\n"))) {
        Ok(_) => {
            println!(
                "{}",
                format!(
                    "Removed entry from hosts file: {} {}",
                    remove.ip.cyan(),
                    remove.hostname.yellow()
                )
                .blue()
            );
        }
        Err(e) => handle_permission_error(e),
    }
}

async fn print_current_entries() {
    let contents = match fs::read_to_string(HOSTS_PATH) {
        Ok(c) => c,
        Err(e) => {
            handle_permission_error(e);
            return;
        }
    };

    for line in contents.lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let ip = parts.first().unwrap_or(&"");
        let hostname = parts.get(1).unwrap_or(&"");

        println!("{} {}", ip.cyan(), hostname.yellow());
    }
}

fn handle_permission_error(err: std::io::Error) {
    if err.kind() == std::io::ErrorKind::PermissionDenied {
        eprintln!(
            "{}",
            "Permission denied. Did you forget to use `sudo`?".red()
        );
    } else {
        eprintln!("Error: {}", err);
    }

    exit(1);
}

#[derive(Parser)]
#[clap(version = crate_version!())]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Add(Add),
    Remove(Remove),
    List,
}

#[derive(Parser)]
struct Add {
    ip: String,
    hostname: String,
}

#[derive(Parser)]
struct Remove {
    ip: String,
    hostname: String,
}

const HOSTS_PATH: &str = "/etc/hosts";
