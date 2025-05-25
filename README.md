# hostie 🛠️

[![Crates.io](https://img.shields.io/crates/v/hostie.svg)](https://crates.io/crates/hostie)
[![GitHub Actions](https://github.com/nrempel/hostie/actions/workflows/rust.yml/badge.svg)](https://github.com/nrempel/hostie/actions)
[![GitHub Releases](https://img.shields.io/github/release/nrempel/hostie.svg)](https://github.com/nrempel/hostie/releases)

hostie is a handy CLI tool that simplifies the process of adding, removing, and
listing entries in your system's hosts file (`/etc/hosts` on Unix, `C:\Windows\System32\drivers\etc\hosts` on Windows).

Perfect for developers who need to quickly map hostnames to IP addresses for local development, testing, or debugging. You no longer need to deal with manual and error-prone editing. Now, it's as simple as running a command.

## Why use hostie?

Instead of manually editing your hosts file like this:

```bash
sudo nano /etc/hosts
# Navigate to the right line, be careful not to break anything...
# Add: 127.0.0.1 myapp.local
# Save and exit
```

Just do this:

```bash
sudo hostie add 127.0.0.1 myapp.local
```

## Features

- 🚀 **Simple commands**: Add, remove, and list hosts entries with ease
- 🛡️ **Safe operations**: Prevents accidental removal of system entries like `localhost`
- 🎯 **Precise matching**: Only exact IP+hostname combinations are affected
- 🌍 **Cross-platform**: Works on macOS, Linux, and Windows
- 📝 **Preserves formatting**: Keeps your hosts file comments and structure intact
- ✅ **Duplicate prevention**: Won't add the same hostname twice
- 🧪 **Well-tested**: 24 comprehensive tests ensure reliability

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

### Basic Usage

**Add a local development site:**

```bash
sudo hostie add 127.0.0.1 myapp.local
```

**Remove an entry when you're done:**

```bash
sudo hostie remove 127.0.0.1 myapp.local
```

**List all current entries:**

```bash
hostie list
```

### Common Development Scenarios

**Set up multiple local services:**

```bash
sudo hostie add 127.0.0.1 api.local
sudo hostie add 127.0.0.1 frontend.local
sudo hostie add 127.0.0.1 admin.local
```

**Point to a staging server:**

```bash
sudo hostie add 192.168.1.100 staging.mycompany.com
```

**Override a production domain for testing:**

```bash
sudo hostie add 127.0.0.1 api.production.com
```

**Block a website (point to localhost):**

```bash
sudo hostie add 127.0.0.1 distracting-website.com
```

### Sample Output

```bash
$ hostie list
127.0.0.1 localhost
127.0.0.1 myapp.local
192.168.1.100 staging.mycompany.com
::1 localhost
```

### What hostie does for you

- ✅ **Prevents duplicates**: Won't add the same hostname twice
- ✅ **Protects system entries**: Can't accidentally remove `localhost`
- ✅ **Preserves formatting**: Keeps comments and empty lines intact
- ✅ **Cross-platform**: Works on macOS, Linux, and Windows
- ✅ **Safe operations**: Only modifies exact matches, no false positives

## Note

You need to use `sudo` to execute the `add` and `remove` commands, as the hosts
file requires administrator privileges to modify its contents.

## License

This project is available under the MIT License.
