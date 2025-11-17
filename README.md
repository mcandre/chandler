# chandler: tar normalizer

![chandler](chandler.png)

# SUMMARY

chandler is a tool for software developers to normalize application tape archives (`*.TGZ`, `*.TAR.GZ` files).

# EXAMPLE

```console
$ cd example

$ tree -p
[drwxr-xr-x]  .
├── [-rw-r--r--]  .gitignore
├── [drwxr-xr-x]  etc
│   ├── [drwxr-xr-x]  init.d
│   │   └── [-rw-r--r--]  ssh
│   └── [-rw-r--r--]  profile
└── [drwxr-xr-x]  hello-1.0.0
    ├── [-rw-r--r--]  hello
    ├── [-rw-r--r--]  hello.bat
    └── [-rw-r--r--]  README

4 directories, 6 files

$ chandler -zvf ../hello-1.0.0.tgz .
a .gitignore
a etc
a etc/init.d
a etc/init.d/ssh
a etc/profile
a hello-1.0.0
a hello-1.0.0/README
a hello-1.0.0/hello
a hello-1.0.0/hello.bat
archived entries to ../hello-1.0.0.tgz

$ tar -tzvf ../hello-1.0.0.tgz
-rw-r--r--  0 1000   1000       22 Nov 13 14:48 .gitignore
drwxr-xr-x  0 0      0           0 Nov 17 14:08 etc
drwxr-xr-x  0 0      0           0 Nov 17 13:57 etc/init.d
-rwxr-xr-x  0 0      0          42 Nov 14 11:17 etc/init.d/ssh
-rw-r--r--  0 0      0          95 Nov 17 14:08 etc/profile
drwxr-xr-x  0 1000   1000        0 Nov 14 11:18 hello-1.0.0
-rw-r--r--  0 1000   1000      186 Nov 13 14:48 hello-1.0.0/README
-rwxr-xr-x  0 1000   1000       31 Nov 13 14:35 hello-1.0.0/hello
-rw-r--r--  0 1000   1000       22 Nov 13 14:34 hello-1.0.0/hello.bat
```

Above, chandler aligns target file metadata to industry standards, repairing glitches in source file metadata.

See `chandler -h` for more options.

# ABOUT

chandler automates industry norms for file permissions, file exclusions, lexicographical sorting, file path normalization, and more.

Metadata is normalized as each entry enters the archive, regardless of the original file metadata. This smooths out common SDLC workflows, especially for multi-platform engineering teams.

# DEFAULT RULES

chandler's default rule set is tuned for application release archives, especially native UNIX executables, Windows executables, and/or interpreted scripts. While also allowing for nonexecutable assets.

* **Assume executable permissions for child paths without a period (`.`) by default**
* Drop executable permissions for common nonexecutable files
* Normalize user and group ID's
* Skip common junk files

Extensionless files are generally assumed to be UNIX executables. These automatically receive `chmod 0755` permissions suitable for running commands.

Executables includes compilied binary programs (`hello`), interpreted text scripts (`configure`), and legacy SysVinit scripts (`etc/init.d/sshd`).

Executables often includes directories, as these permissions are requisite in order for relevant users to perform directory traversal.

Common extensionless basenames are recognized as *nonexecutable* (`LICENSE`, `README`, `makefile`, etc.)

Child paths with period (`.`) are assumed to be nonexecutable assets, receiving `chmod 0644` permissions.

This includes Windows centric programs (`hello.bat`, `*.com`, `*.exe`, etc.), scripts that include a file extension (`*.bat`, `*.cmd`, `*.js`, `*.lua`, `*.pl`, `*.py`, `*.rb`, `*.sh`, etc.), as well as documents (`*.json`, `*.md`, `*.toml`, `*.txt`, `*.xml`, `*.yaml`, `*.yml`, etc.), archives (`*.jar`, `*.tar`, `*.tar.gz`, `*.tgz`, `*.zip`, etc.), and most descendents of the UNIX configuration directory `etc`.

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
* case sensitive or case aware file systems (e.g. ext4, exFAT, APFS, NTFS)
* tar with gzip support (e.g., [GNU](https://www.gnu.org/software/tar/manual/tar.html)/[BSD](https://man.freebsd.org/cgi/man.cgi?tar(1))/[Windows](https://ss64.com/nt/tar.html))
* [tree](https://en.wikipedia.org/wiki/Tree_(command))

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
