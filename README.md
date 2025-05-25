# hostie 🛠️

[![Crates.io](https://img.shields.io/crates/v/hostie.svg)](https://crates.io/crates/hostie)
[![GitHub Actions](https://github.com/nrempel/hostie/actions/workflows/rust.yml/badge.svg)](https://github.com/nrempel/hostie/actions)
[![GitHub Releases](https://img.shields.io/github/release/nrempel/hostie.svg)](https://github.com/nrempel/hostie/releases)

hostie is a handy CLI tool that simplifies the process of adding, removing, and
listing entries in the hosts file. You no longer need to deal with manual and
error-prone editing. Now, it's as simple as running a command.

## Features

- Add entries to the hosts file
- Remove entries from the hosts file
- Print the current entries in the hosts file

## Installation

### Download Compiled Binaries

You can download the compiled binaries for hostie from the
[GitHub Releases](https://github.com/nrempel/hostie/releases) page. Choose the
binary that corresponds to your operating system and architecture, and place it
in a directory included in your system's `PATH` environment variable.

### Install with Cargo

To install hostie using Cargo, you'll need to have
[Rust](https://www.rust-lang.org/tools/install) installed on your system. Once
Rust is installed, you can install hostie with Cargo:

```bash
cargo install hostie
```

## Usage

```bash
hostie [COMMAND]
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
sudo hostie add 127.0.0.1 dev.local
```

Remove an entry from the hosts file:

```bash
sudo hostie remove 127.0.0.1 dev.local
```

Print the current entries in the hosts file:

```bash
hostie list
```

## Note

You need to use `sudo` to execute the `add` and `remove` commands, as the hosts
file requires administrator privileges to modify its contents.

## License

This project is available under the MIT License.
