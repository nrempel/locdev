use std::fs::{self, OpenOptions, read_to_string};
use std::io::prelude::*;
use std::process::ExitCode;

use clap::Parser;
use colored::{ColoredString, Colorize};
use thiserror::Error;

fn main() -> ExitCode {
    let opts: Options = Options::parse();

    let result = match opts.subcmd {
        SubCommand::Add(add) => add_hosts_entry(&add),
        SubCommand::Remove(remove) => remove_hosts_entry(remove),
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
    let new_entry_line = format!("{} {}", add.ip, add.hostname);

    let contents = read_to_string(get_hosts_path())?;

    // Check for exact hostname match (not just ends_with)
    let hostname_exists = contents
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .any(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            parts.get(1).is_some_and(|h| h == &add.hostname)
        });

    if hostname_exists {
        return Err(Error::Generic(
            format!("Entry already exists: {new_entry}").red(),
        ));
    }

    let mut file = OpenOptions::new().append(true).open(get_hosts_path())?;
    file.write_all(format!("{}\n", new_entry_line).as_bytes())?;

    Ok(format!("Added entry to hosts file: {new_entry}").green())
}

fn remove_hosts_entry(remove: AddRemove) -> Result<ColoredString, Error> {
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

    let contents = read_to_string(get_hosts_path())?;

    let entry_to_remove = format!(
        "{} {}",
        remove.ip.cyan().bold(),
        remove.hostname.magenta().bold()
    );
    let entry_to_remove_line = format!("{} {}", remove.ip, remove.hostname);

    // Check for exact IP+hostname match
    let entry_exists = contents
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .any(|line| {
            let normalized = line.split_whitespace().collect::<Vec<_>>().join(" ");
            normalized == entry_to_remove_line
        });

    if !entry_exists {
        return Err(Error::Generic(
            format!("Entry does not exist: {entry_to_remove}").red(),
        ));
    }

    // Remove only the exact matching line
    let entries: Vec<_> = contents
        .lines()
        .filter(|line| {
            if line.trim().is_empty() || line.starts_with('#') {
                true // Keep comments and empty lines
            } else {
                let normalized = line.split_whitespace().collect::<Vec<_>>().join(" ");
                normalized != entry_to_remove_line
            }
        })
        .collect();

    // Write back with proper formatting (single newline at end)
    let new_content = if entries.is_empty() {
        String::new()
    } else {
        format!("{}\n", entries.join("\n"))
    };

    fs::write(get_hosts_path(), new_content)?;

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
#[command(author, version, about, long_about = None)]
struct Options {
    #[command(subcommand)]
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
    /// The IP address to use
    #[arg(value_name = "IP")]
    ip: String,

    /// The hostname to associate with the IP address
    #[arg(value_name = "HOSTNAME")]
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
    std::env::var("HOSTIE_HOSTS_FILE").unwrap_or_else(|_| {
        if cfg!(windows) {
            r"C:\Windows\System32\drivers\etc\hosts".to_string()
        } else {
            "/etc/hosts".to_string()
        }
    })
}
