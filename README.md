# hostman 🛠️

[![Crates.io](https://img.shields.io/crates/v/hostman.svg)](https://crates.io/crates/hostman)
[![GitHub Actions](https://github.com/nrempel/hostman/actions/workflows/rust.yml/badge.svg)](https://github.com/nrempel/hostman/actions)
[![GitHub Releases](https://img.shields.io/github/release/nrempel/hostman.svg)](https://github.com/nrempel/hostman/releases)

hostman is a handy CLI tool that simplifies the process of adding, removing, and
listing entries in the hosts file. You no longer need to deal with manual and
error-prone editing. Now, it's as simple as running a command.

## Features

- Add entries to the hosts file
- Remove entries from the hosts file
- Print the current entries in the hosts file

## Installation

### Download Compiled Binaries

You can download the compiled binaries for hostman from the
[GitHub Releases](https://github.com/nrempel/hostman/releases) page. Choose the
binary that corresponds to your operating system and architecture, and place it
in a directory included in your system's `PATH` environment variable.

### Install with Cargo

To install hostman using Cargo, you'll need to have
[Rust](https://www.rust-lang.org/tools/install) installed on your system. Once
Rust is installed, you can install hostman with Cargo:

```bash
cargo install hostman
```

## Usage

```bash
hostman [COMMAND]
```

### Commands

- `add <ip> <hostname>`: Add an entry to the hosts file with the specified IP
  and hostname
- `remove <ip> <hostname>`: Remove the entry with the specified IP and hostname
  from the hosts file
- `list`: Print the current entries in the hosts file

## Examples

Add an entry to the hosts file:

```bash
sudo hostman add 127.0.0.1 dev.local
```

Remove an entry from the hosts file:

```bash
sudo hostman remove 127.0.0.1 dev.local
```

Print the current entries in the hosts file:

```bash
hostman list
```

## Note

You need to use `sudo` to execute the `add` and `remove` commands, as the hosts
file requires administrator privileges to modify its contents.

## License

This project is available under the MIT License.
