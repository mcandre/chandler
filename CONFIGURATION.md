# CONFIGURATION

chandler loads an optional `chandler.toml` file in the current working directory.

## Example

```toml
# verbose = true

# header.type = "UStar"

# cwd = "..."

# skip_paths = [
#     ".DS_Store",
#     "Thumbs.db",
# ]

# [[rules]]
# permissions = 0o755
#
# [[rules]]
# when.mode.type = "File"
# when.paths = "(?i)^aliases|(ba|(m)?k|z)shrc|(bsd|gnu)?makefile|changelog|exports|fstab|license|readme|group|hosts|issue|mime|modules|profile|protocols|resolv|services|t(e)?mp|zshenv|((.*/)?etc/.+)$"
# permissions = 0o644
#
# [[rules]]
# when.mode.type = "File"
# when.paths = "^(.*/)*[^/]*\\.[^/]*$"
# permissions = 0o644
#
# [[rules]]
# when.paths = "^(.*/)?etc/init\\.d(/.*)?$"
# permissions = 0o755
```

# verbose

Default: `false`.

When `true`, enables additional logging.

# header.type

Default: `UStar`.

Controls the inner TAR format.

Supported TAR formats:

* `UStar` - Modern, POSIX compliant tarballs
* `Gnu` - Classical, GNU tarballs
* `TarV7` - Vintage tarballs

# cwd

Default: The working directory of the shell that invokes `chandler`.

When customized, chandler adjusts its current working directory to some given location. This helps, for example, to build clean tarballs when the original files are nested within a larger project directory.

Note that it is often good practice to preserve a parent top level directory for each tarball entry, so that the files don't explode to interleave with user files, when the archive expands.

# skip_paths

Default:

```rust
[
    ".DS_Store",
    "Thumbs.db",
]
```

skip_paths collects Rust [regex](https://crates.io/crates/regex) patterns for excluding entries from archival.

# rules

Default:

1. Assume `chmod 0755` permissions by default.
2. Apply `chmod 0644` permissions for common, nonexecutable files.
3. Apply `chmod 0644` permissions for filenames that include a file extension.
4. Apply `chmod 0755` permissions for legacy SysVinit files.

Rules define behaviors for archive metadata, such as tuning `chown` and `chmod` permissions.

For more information on rules, see [Rules](https://docs.rs/chandler/0.0.3/chandler/struct.Rule.html).
