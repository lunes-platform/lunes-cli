# Lunes CLI

[![Build](https://github.com/lunes-platform/lunes-cli/actions/workflows/build.yml/badge.svg)](https://github.com/lunes-platform/lunes-cli/actions/workflows/build.yml)
[![Test](https://github.com/lunes-platform/lunes-cli/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/lunes-platform/lunes-cli/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/lunes-platform/lunes-cli)](https://github.com/lunes-platform/lunes-cli/releases)

- CLI Manager for Lunes Full-Node write in Rust

## How to Install

## Linux

*Download the `lunes-linux` [here](https://github.com/lunes-platform/lunes-cli/releases)*

*Move to /bin/ and rename to `lunes`*
```
mv lunes-linux /usr/bin/lunes
```

*Allow its execution*
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
