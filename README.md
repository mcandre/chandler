# chandler: tar normalizer

![chandler](chandler.png)

# SUMMARY

chandler is a tool for software developers to normalize application tape archives (`*.TGZ`, `*.TAR.GZ` files).

chandler automates industry norms for file permissions, file exclusions, lexicographical sorting, and more.

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

## Debrief

chandler aligns file permissions as files enter the tarball.

Most files are assumed to be UNIX executables, e.g., `hello`, and automatically receive `chmod 0755` permissions suitable for running commands.

Windows programs, e.g. `hello.bat`, `*.exe`, etc. reduce permissions to more appropriate `chmod 0644`. As do non-application assets, e.g. `README.md`, `*.txt`, etc.

The metadata is corrected in the archive, regardless of the original file metadata. This smooths out common SDLC workflows, creating multi-platform engineering teams.

See `chandler -h` for more options.

# ABOUT

chandler's default rule set is tuned to support precompiled executable archives for UNIX and/or Windows applications. While also allowing archives to include nonexecutable entries.

Note: The term *executable* may include binary applications as well as shell script commands.

# DEFAULT RULES

* **Assume executable permissions by default**
* Drop executable permissions for Windows executables
* Drop executable permissions for common non-application assets
* Normalize user and group ID's
* Skip common junk files

Rules like these help to reduce glitches during software builds. For example, when manipulating binaries or shell scripts with Windows environments.

By carefully adjusting metadata as build artifacts enter the archive, we enable incredibly portable build systems. So that more engineers can contribute to projects, regardless of whether they're using UNIX or Windows development environments.

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
