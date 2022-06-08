# Lunes CLI

🕹 CLI Manager for Lunes Full-Node write in Rust

[![Build](https://github.com/lunes-platform/lunes-cli/actions/workflows/build.yml/badge.svg)](https://github.com/lunes-platform/lunes-cli/actions/workflows/build.yml)
[![Test](https://github.com/lunes-platform/lunes-cli/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/lunes-platform/lunes-cli/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/lunes-platform/lunes-cli)](https://github.com/lunes-platform/lunes-cli/releases)

## Install by Rust

**needs [Rust]()**
```
cargo install lunes
```

## Install by Linux

_Download the `lunes cli` [here](https://github.com/lunes-platform/lunes-cli/releases)_

*Move to /usr/bin/ and rename to `lunes`*
```
mv lunes-*** /usr/bin/lunes
```

_Allow its execution_

```
chmod +x /usr/bin/lunes
```

<!--
## Windowns

1. download the `lunes-windows.exe` [here](https://github.com/lunes-platform/lunes-cli/releases)
2. `rename lunes-windowns.exe lunes.exe`
3. move `lunes.exe` to `C:\Windows\lunes.exe`
4. open start menu:
   - search for **edit environment variables** and open
   - click in **environment variables** > **system variables** > **new**
   - **variable name:** `lunes`
   - **variable value:** `C:\Windows\lunes.exe`
5. **restart the command prompt** -->

## How to Use

```
lunes
🕹 Lunes CLI management for full-node and wallet

USAGE:
    lunes <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    node      🌎 Management your Lunes Node
    wallet    🔑 Management your Lunes Wallet
```
