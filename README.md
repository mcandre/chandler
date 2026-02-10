# chandler: GNU tar normalizer

[![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/chandler)](https://hub.docker.com/r/n4jm4/chandler) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/chandler?label=crate%20downloads&labelColor=grey&color=green)](https://crates.io/crates/chandler) [![docs.rs](https://img.shields.io/docsrs/chandler)](https://docs.rs/chandler/latest/chandler/) [![Test](https://github.com/mcandre/chandler/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/chandler/actions/workflows/test.yml) [![Test-Futureproof-Dependencies](https://github.com/mcandre/chandler/actions/workflows/test-futureproof-dependencies.yml/badge.svg)](https://github.com/mcandre/chandler/actions/workflows/test-futureproof-dependencies.yml) [![Test-Futureproof-Language](https://github.com/mcandre/chandler/actions/workflows/test-futureproof-language.yml/badge.svg)](https://github.com/mcandre/chandler/actions/workflows/test-futureproof-language.yml) [![Test-Futureproof-OS](https://github.com/mcandre/chandler/actions/workflows/test-futureproof-os.yml/badge.svg)](https://github.com/mcandre/chandler/actions/workflows/test-futureproof-os.yml) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md) [![Donate](https://img.shields.io/badge/GUMROAD-36a9ae?style=flat&logo=gumroad&logoColor=white)](https://mcandre.gumroad.com/)

![chandler](chandler.png)

# SUMMARY

chandler is a tool for software developers to normalize application tape archives (`*.TGZ`, `*.TAR.GZ` files).

# EXAMPLE

```console
$ cd example

$ tree -p hello-1.0.0
[drwxr-xr-x]  hello-1.0.0
├── [-rw-r--r--]  hello
├── [-rw-r--r--]  hello.bat
└── [-rw-r--r--]  README

1 directory, 3 files

$ chandler -czf hello-1.0.0.tgz hello-1.0.0
archived entries to hello-1.0.0.tgz

$ tar -tzvf hello-1.0.0.tgz
drwxr-xr-x  0 501    20          0 Nov 14 11:18 hello-1.0.0
-rw-r--r--  0 501    20        186 Nov 13 14:48 hello-1.0.0/README
-rwxr-xr-x  0 501    20         31 Nov 13 14:35 hello-1.0.0/hello
-rw-r--r--  0 501    20         22 Nov 13 14:34 hello-1.0.0/hello.bat
```

Above, chandler aligns target file metadata to industry standards, repairing glitches in source file metadata.

See [CONFIGURATION.md](CONFIGURATION.md) for configuration file options.

Run `chandler -h` for CLI options.

# ABOUT

chandler automates industry norms for file permissions, file exclusions, lexicographical sorting, file path normalization, and more.

Metadata is normalized as each entry enters the archive, regardless of the original file metadata. This smooths out common SDLC workflows, especially for multi-platform engineering teams.

# INSTALLATION

See [INSTALL.md](INSTALL.md).

## Recommended

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* case sensitive or case aware file systems (e.g. ext4, exFAT, APFS, NTFS)
* [GNU](https://www.gnu.org/software/tar/)/[BSD](https://man.freebsd.org/cgi/man.cgi?tar(1))/[Windows](https://ss64.com/nt/tar.html) tar with gzip support
* [tree](https://en.wikipedia.org/wiki/Tree_(command))

# SEE ALSO

* [factorio](https://github.com/mcandre/factorio)
* [crit](https://github.com/mcandre/crit)
* [linters](https://github.com/mcandre/linters)
* [rockhopper](https://github.com/mcandre/rockhopper)
* [tinyrick](https://github.com/mcandre/tinyrick)
* [todolint](https://github.com/mcandre/todolint)
* [tuggy](https://github.com/mcandre/tuggy)

📼
