# chandler: tar normalizer

![chandler](chandler.png)

# SUMMARY

chandler is a tool for software developers to normalize application tape archives (`*.TGZ`, `*.TAR.GZ` files).

# EXAMPLE

```console
$ cd example

$ ls -l hello-1.0.0
total 24
-rw-r--r--  1 andrew  staff   31 Nov 13 14:35 hello
-rw-r--r--  1 andrew  staff   22 Nov 13 14:34 hello.bat
-rw-r--r--  1 andrew  staff  186 Nov 13 14:48 README.md

$ chandler -f hello-1.0.0.tgz hello-1.0.0
archived entries to hello-1.0.0.tgz

$ tar tzvf hello-1.0.0.tgz
drwxr-xr-x  0 1000   1000        0 Nov 13 14:36 hello-1.0.0
-rw-r--r--  0 1000   1000      170 Nov 13 14:38 hello-1.0.0/README.md
-rwxr-xr-x  0 1000   1000       31 Nov 13 14:35 hello-1.0.0/hello
-rw-r--r--  0 1000   1000       22 Nov 13 14:34 hello-1.0.0/hello.bat
```

See `chandler -h` for more options.

# ABOUT

chandler automates industry norms for file permissions, file exclusions, lexicographical sorting, and more.

Extensionless files are generally assumed to be UNIX executables (`hello`, etc.) These automatically receive `chmod 0755` permissions suitable for running commands.

Child paths with period (`.`) are assumed to be nonexecutable assets, receiving `chmod 0644` permissions. This includes Windows centric programs (`hello.bat`, `*.exe`, etc.), scripts that include a file extension (`*.js`, `*.lua`, `*.pl`, `*.py`, `*.rb`, `*.sh`, etc.), as well as documents (`*.json`, `*.md`, `*.toml`, `*.txt`, `*.xml`, `*.yaml`, `*.yml`, etc.), and archives (`*.tar`, `*.tar.gz`, `*.tgz`, `*.zip`, etc.)

Metadata is normalized as each entry enters the archive, regardless of the original file metadata. This smooths out common SDLC workflows, especially for multi-platform engineering teams.

Note: The term *executable* may include binary applications as well as shell script commands.

# DEFAULT RULES

chandler's default rule set is tuned for application release archives, especially native UNIX executables, Windows executables, and/or interpreted scripts. While also allowing for nonexecutable assets.

* **Assume executable permissions for child paths without a period (`.`) by default**
* Drop executable permissions for common nonexecutable files
* Normalize user and group ID's
* Skip common junk files

# CRATE

https://crates.io/crates/chandler

# API DOCUMENTATION

https://docs.rs/chandler/latest/chandler/

# DOWNLOAD

https://github.com/mcandre/chandler/releases

# INSTALL FROM SOURCE

```console
$ cargo install --force --path .
```

# RUNTIME REQUIREMENTS

(None)

## Recommended

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* POSIX compliant [tar](https://pubs.opengroup.org/onlinepubs/7908799/xcu/tar.html)

# CONTRIBUTING

For more details on developing unmake itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# LICENSE

BSD-2-Clause

# SEE ALSO

* [factorio](https://github.com/mcandre/factorio)
* [crit](https://github.com/mcandre/crit)
* [linters](https://github.com/mcandre/linters)
* [tug](https://github.com/mcandre/tug)

📼
