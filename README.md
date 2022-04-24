# Lunes CLI

[![Build](https://github.com/lunes-platform/lunes-cli/actions/workflows/build.yml/badge.svg)](https://github.com/lunes-platform/lunes-cli/actions/workflows/build.yml)
[![Test](https://github.com/lunes-platform/lunes-cli/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/lunes-platform/lunes-cli/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/lunes-platform/lunes-cli)](https://github.com/lunes-platform/lunes-cli/releases)

- CLI Manager for Lunes Full-Node write in Rust

## How to Install

### Linux

1. download the `tasker-linux` [here](https://github.com/lunes-platform/lunes-cli/releases)
2. `mv tasker-linux /usr/bin/tasker`
3. `chmod +x /usr/bin/taker`

### Windows

1. download the `tasker-windows.exe` [here](https://github.com/lunes-platform/lunes-cli/releases)
2. `rename tasker-windowns.exe tasker.exe`
3. move `tasker.exe` to `C:\Windows\tasker.exe`
4. open start menu:
   - search for **edit environment variables** and open
   - click in **environment variables** > **system variables** > **new**
   - **variable name:** `tasker`
   - **variable value:** `C:\Windows\tasker.exe`
5. **restart the command prompt**

## How to Use

```
lunes

    node
        config [KEY_1=VALUE_1] [KEY_2=VALUE_2] [...]
        install [--version=0.0.1]
        status
        logs
        down
        up

    wallet
        new
        add [name=seed] [-p [name=private_key]]
        rename [old_name=new_name]
        remove [name]
        [name]
        list

```
