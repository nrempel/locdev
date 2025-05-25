use std::fs::{self, OpenOptions, read_to_string};
use std::io::prelude::*;
use std::process::ExitCode;

use clap::{Parser, crate_version};
use colored::{ColoredString, Colorize};
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> ExitCode {
    let opts: Options = Options::parse();

    let result = match opts.subcmd {
        SubCommand::Add(add) => add_hosts_entry(&add),
        SubCommand::Remove(remove) => remove_hosts_entry(remove).await,
        SubCommand::List => print_current_entries(),
    };

    match result {
        Ok(msg) => {
            println!("{msg}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn add_hosts_entry(add: &AddRemove) -> Result<ColoredString, Error> {
    let new_entry = format!("{} {}", add.ip.cyan().bold(), add.hostname.magenta().bold());
    let new_entry_line = format!("{} {}\n", add.ip, add.hostname);

    let contents = read_to_string(get_hosts_path())?;
    if contents.lines().any(|line| line.ends_with(&add.hostname)) {
        return Err(Error::Generic(
            format!("Entry already exists: {new_entry}").red(),
        ));
    }

    let mut file = OpenOptions::new().append(true).open(get_hosts_path())?;
    file.write_all(new_entry_line.as_bytes())?;

    Ok(format!("Added entry to hosts file: {new_entry}").green())
}

async fn remove_hosts_entry(remove: AddRemove) -> Result<ColoredString, Error> {
    let protected_hostnames = ["localhost", "broadcasthost"];

    if protected_hostnames.contains(&remove.hostname.as_str()) {
        return Err(Error::Generic(
            format!(
                "Cannot remove protected entry: {}",
                remove.hostname.magenta().bold()
            )
            .red(),
        ));
    }

    let mut hosts_file = File::open(get_hosts_path()).await?;
    let mut contents = String::new();
    hosts_file.read_to_string(&mut contents).await?;

    let entry_to_remove = format!(
        "{} {}",
        remove.ip.cyan().bold(),
        remove.hostname.magenta().bold()
    );
    let entry_to_remove_line = format!("{} {}", remove.ip, remove.hostname);
    let entry_exists = contents.lines().any(|line| line == entry_to_remove_line);

    if !entry_exists {
        return Err(Error::Generic(
            format!("Entry does not exist: {entry_to_remove}").red(),
        ));
    }

    let entries: Vec<_> = contents
        .lines()
        .filter(|line| !line.ends_with(&remove.hostname))
        .collect();

    fs::write(get_hosts_path(), format!("{}\n", entries.join("\n")))?;

    Ok(format!(
        "Removed entry from hosts file: {} {}",
        remove.ip.cyan().bold(),
        remove.hostname.magenta().bold()
    )
    .green())
}

fn print_current_entries() -> Result<ColoredString, Error> {
    let contents = read_to_string(get_hosts_path())?;

    let current_entries = contents
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .map(|e| {
            let parts: Vec<&str> = e.split_whitespace().collect();
            let ip = parts.first().unwrap_or(&"");
            let hostname = parts.get(1).unwrap_or(&"");
            format!("{} {}", ip.cyan().bold(), hostname.magenta().bold())
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(current_entries.green())
}

#[derive(Parser)]
#[clap(version = crate_version!())]
/// A simple CLI tool for managing your /etc/hosts file
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    /// Add a new entry to your hosts file
    Add(AddRemove),
    /// Remove an entry from your hosts file
    Remove(AddRemove),
    /// List all entries in your hosts file
    List,
}

#[derive(Parser)]
struct AddRemove {
    /// The IP address to add
    ip: String,
    /// The hostname to add
    hostname: String,
}

#[derive(Error, Debug)]
enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Generic(ColoredString),
}

fn get_hosts_path() -> String {
    std::env::var("HOSTIE_HOSTS_FILE").unwrap_or_else(|_| "/etc/hosts".to_string())
}
