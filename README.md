# Lunes CLI

[![Build](https://github.com/lunes-platform/lunes-cli/actions/workflows/build.yml/badge.svg)](https://github.com/lunes-platform/lunes-cli/actions/workflows/build.yml)
[![Test](https://github.com/lunes-platform/lunes-cli/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/lunes-platform/lunes-cli/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/lunes-platform/lunes-cli)](https://github.com/lunes-platform/lunes-cli/releases)

- CLI Manager for Lunes Full-Node write in Rust

## How to Install

## Linux

_Download the `lunes-linux` [here](https://github.com/lunes-platform/lunes-cli/releases)_

_Move to /bin/ and rename to `lunes`_

```
mv lunes-linux /usr/bin/lunes
```

_Allow its execution_

```
chmod +x /usr/bin/taker
```

## Windows

```
Comming Soon
```

<!-- 1. download the `lunes-windows.exe` [here](https://github.com/lunes-platform/lunes-cli/releases)
2. `rename lunes-windowns.exe lunes.exe`
3. move `lunes.exe` to `C:\Windows\lunes.exe`
4. open start menu:
   - search for **edit environment variables** and open
   - click in **environment variables** > **system variables** > **new**
   - **variable name:** `lunes`
   - **variable value:** `C:\Windows\lunes.exe`
5. **restart the command prompt** -->

## How to Use

## Basic Commands

```
lunes
Lunes CLI management for full-node and wallet

USAGE:
    lunes <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    node    Commands to management your Lunes Node
```

## Commands for Full-Node

```
lunes-node
Commands to management your Lunes Node

USAGE:
    lunes node
    lunes node <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    config     Comming Soon
    down       Shutdown your Lunes Node
    help       Print this message or the help of the given subcommand(s)
    install    Comming Soon
    logs       Follow your Lunes Node logs
    restart    Restart your your Lunes Node
    status     Status of your Lunes Node
    up         Turn On your Lunes Node
    version    Comming Soon
```
