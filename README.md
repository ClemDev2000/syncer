# Syncer 🖇

File copying tool to sync directories. Built with Rust 🦀 for security 🔐 and speed ⚡️.

## Installation

```bash
curl -L https://raw.githubusercontent.com/ClemDev2000/syncer/main/download-latest.sh | sh
```

## Examples

**⚠️ Syncer will always delete the files from `slave` that are not present in `root`. Proceed with caution to avoid data loss.**

Copy files from root to slave:

```bash
./syncer /path/to/root /path/to/slave
```

Help:

```bash
./syncer --help
```

## Build from source

> To build the project on your computer ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

Clone the repo:

```bash
git clone https://github.com/ClemDev2000/syncer
```

Build in release mode:

```bash
cargo build --release
```

Execute the binary:

```bash
./target/release/syncer
```
