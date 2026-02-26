# INSTALL GUIDE

In addition to curl, chandler also supports alternative installation methods.

# INSTALL (DOCKER)

chandler is packaged as a Docker image.

```sh
docker pull n4jm4/chandler
```

## Prerequisites

* [Docker](https://www.docker.com/)

# INSTALL (CARGO)

chandler is packaged as a Rust crate.

```sh
cargo install chandler
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)

# INSTALL (PRECOMPILED BINARIES)

Precompiled binaries may be installed manually.

## Install

1. Download a [tarball](https://github.com/mcandre/chandler/releases) corresponding to your environment's architecture and OS.
2. Extract executables into a selected directory.

   Examples:

   * `~/.local/bin` (XDG compliant per-user)
   * `/usr/local/bin` (XDG compliant global)
   * `~/bin` (BSD)
   * `~\AppData\Local` (native Windows)

## Postinstall

Ensure the selected directory is registered with your shell's `PATH` environment variable.

## Uninstall

Remove the application executables from the selected directory.

## System Requirements

### Bitness

64

### Hosts

* FreeBSD (Intel)
* Illumos (Intel)
* Linux (ARM, Intel)
* macOS (ARM, Intel)
* NetBSD (Intel)
* Windows (ARM, Intel) native or [WSL](https://learn.microsoft.com/en-us/windows/wsl/)

# INSTALL (COMPILE FROM SOURCE)

chandler may be compiled from source.

```sh
git clone https://github.com/mcandre/chandler.git
cd chandler
cargo install --force --path .
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)
* [git](https://git-scm.com/)

For more details on developing chandler, see our [development guide](DEVELOPMENT.md).
