//! chandler assembles tape archives.

extern crate fancy_regex;
extern crate flate2;
extern crate normalize_path;
extern crate serde;
extern crate tar;
extern crate toml;
extern crate walkdir;

use self::serde::{Deserialize, Serialize};
use normalize_path::NormalizePath;

use std::env;
use std::fs;
use std::io;
use std::path;
use std::sync;
use std::time;

/// COMMON_SKIP_PATHS collects file paths commonly excluded from clean archives,
/// such as file manager metadata files.
pub static COMMON_SKIP_PATHS: sync::LazyLock<Vec<&str>> =
    sync::LazyLock::new(|| vec![".DS_Store", "Thumbs.db"]);

/// SKIP_PATH_PATTERN_TEMPLATE combines with `skip_paths` and a pipe (|) delimited path string to form path exclusion patterns.
pub static SKIP_PATH_PATTERN_TEMPLATE: &str = r"^(.*/)?({skip_paths})$";

/// generate_skip_path_pattern converts a collection of skip paths to a regex.
#[allow(clippy::result_large_err)]
pub fn generate_skip_path_pattern(
    skip_paths: &[&str],
) -> Result<fancy_regex::Regex, fancy_regex::Error> {
    fancy_regex::Regex::new(
        &SKIP_PATH_PATTERN_TEMPLATE.replace("{skip_paths}", &skip_paths.join("|")),
    )
}

/// DEFAULT_SKIP_PATH_PATTERN excludes COMMON_SKIP_FILES.
pub static DEFAULT_SKIP_PATH_PATTERN: sync::LazyLock<fancy_regex::Regex> =
    sync::LazyLock::new(|| generate_skip_path_pattern(&COMMON_SKIP_PATHS).unwrap());

#[test]
fn test_default_skip_path_pattern() -> Result<(), fancy_regex::Error> {
    let pattern = &*DEFAULT_SKIP_PATH_PATTERN;
    assert!(pattern.is_match(".DS_Store")?);
    assert!(pattern.is_match("docs/.DS_Store")?);
    assert!(pattern.is_match("/docs/.DS_Store")?);
    assert!(!pattern.is_match("docs")?);
    assert!(!pattern.is_match("/docs")?);
    assert!(pattern.is_match("Thumbs.db")?);
    assert!(pattern.is_match("docs/Thumbs.db")?);
    Ok(())
}

/// EXTENSIONED_FILE_PATH_PATTERN matches file paths with extensions,
/// including file extensions (.BAT, .EXE, and so on),
/// as well as file paths missing traditional basenames (.gitignore, .git, and so on).
pub static EXTENSIONED_FILE_PATH_PATTERN: sync::LazyLock<fancy_regex::Regex> =
    sync::LazyLock::new(|| fancy_regex::Regex::new(r"^(.*/)*[^/]*\.[^/]*$").unwrap());

/// SYSTEM_V_INIT_LINEAGE_PATTERN matches file paths within SysVinit (etc/init.d) directory trees.
pub static SYSTEM_V_INIT_LINEAGE_PATTERN: sync::LazyLock<fancy_regex::Regex> =
    sync::LazyLock::new(|| fancy_regex::Regex::new(r"^(.*/)?etc/init\.d(/.*)?$").unwrap());

pub static COMMON_NONEXECUTABLE_FILE_PATH_PATTERN: sync::LazyLock<fancy_regex::Regex> =
    sync::LazyLock::new(|| {
        fancy_regex::Regex::new(r"(?i)^aliases|(ba|(m)?k|z)shrc|(bsd|gnu)?makefile|changelog|exports|fstab|license|readme|group|hosts|issue|mime|modules|profile|protocols|resolv|services|t(e)?mp|zshenv|((.*/)?etc/.+)$").unwrap()
    });

#[test]
fn test_extensioned_file_path_pattern() -> Result<(), fancy_regex::Error> {
    let pattern = &*EXTENSIONED_FILE_PATH_PATTERN;
    assert!(!pattern.is_match("hello")?);
    assert!(!pattern.is_match("HELLO")?);
    assert!(!pattern.is_match("hello-1.0/docs")?);
    assert!(pattern.is_match("HELLO.BAT")?);
    assert!(pattern.is_match("hello.bat")?);
    assert!(pattern.is_match("applications/hello.bat")?);
    assert!(pattern.is_match("HELLO.EXE")?);
    assert!(pattern.is_match("hello.exe")?);
    assert!(pattern.is_match("applications/hello.exe")?);
    assert!(pattern.is_match(".gitignore")?);
    assert!(pattern.is_match("DEGENERATE.")?);
    assert!(pattern.is_match("degenerate.")?);
    Ok(())
}

#[test]
fn test_system_v_init_lineage_pattern() -> Result<(), fancy_regex::Error> {
    let pattern = &*SYSTEM_V_INIT_LINEAGE_PATTERN;
    assert!(pattern.is_match("/etc/init.d")?);
    assert!(pattern.is_match("etc/init.d")?);
    assert!(pattern.is_match("/etc/init.d/ssh")?);
    assert!(pattern.is_match("etc/init.d/ssh")?);
    assert!(!pattern.is_match("/root/.ssh")?);
    assert!(!pattern.is_match("root/.ssh")?);
    Ok(())
}

#[test]
fn test_common_nonexecutable_file_path_pattern() -> Result<(), fancy_regex::Error> {
    let pattern = &*COMMON_NONEXECUTABLE_FILE_PATH_PATTERN;
    assert!(pattern.is_match("bashrc")?);
    assert!(pattern.is_match("bsdmakefile")?);
    assert!(pattern.is_match("changelog")?);
    assert!(pattern.is_match("gnumakefile")?);
    assert!(pattern.is_match("license")?);
    assert!(pattern.is_match("makefile")?);
    assert!(pattern.is_match("README")?);
    assert!(pattern.is_match("readme")?);
    assert!(pattern.is_match("aliases")?);
    assert!(pattern.is_match("exports")?);
    assert!(pattern.is_match("fstab")?);
    assert!(pattern.is_match("group")?);
    assert!(pattern.is_match("hosts")?);
    assert!(pattern.is_match("issue")?);
    assert!(pattern.is_match("kshrc")?);
    assert!(pattern.is_match("mime")?);
    assert!(pattern.is_match("mkshrc")?);
    assert!(pattern.is_match("modules")?);
    assert!(pattern.is_match("profile")?);
    assert!(pattern.is_match("protocols")?);
    assert!(pattern.is_match("resolv")?);
    assert!(pattern.is_match("services")?);
    assert!(pattern.is_match("temp")?);
    assert!(pattern.is_match("tmp")?);
    assert!(pattern.is_match("zshenv")?);
    assert!(pattern.is_match("zshrc")?);
    assert!(pattern.is_match("/etc/sshd/sshd_config")?);
    assert!(pattern.is_match("etc/sshd/sshd_config")?);
    Ok(())
}

/// HeaderType models a tarball header type.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum HeaderType {
    /// Old models a vintage tar v7 header.
    Old,

    /// Gnu models a classical GNU tar header.
    Gnu,

    /// UStar models a POSIX UStar/PAX header.
    UStar,
}

/// HeaderType models a tarball header type.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum FileMode {
    /// Directory models a folder.
    Directory,

    /// File models an ordinary, non-directory file.
    File,
}

/// Condition models an archive entry state.
///
/// Fields with values present are intersected together (AND).
#[derive(Debug)]
pub struct Condition<'a> {
    /// mode denotes an FileMode.
    pub mode: Option<FileMode>,

    /// path denotes a file path.
    pub path: Option<&'a fancy_regex::Regex>,
}

/// Rule applies given permissions for matching file patterns.
#[derive(Debug)]
pub struct Rule<'a> {
    /// when denotes a condition required to apply this rule's effects.
    pub when: Condition<'a>,

    /// mtime overrides entry modification timestamps (UNIX epoch).
    pub mtime: Option<u64>,

    /// uid denotes an effective user id.
    pub uid: Option<u64>,

    /// gid denotes an effective group id.
    pub gid: Option<u64>,

    /// username denotes an effective username.
    pub username: Option<String>,

    /// groupname denotes an effective group name.
    pub groupname: Option<String>,

    /// permissions denotes an effective chmod mask of file permissions.
    pub permissions: Option<u32>,
}

/// DEFAULT_RULES implements common archive entry behaviors,
/// such as marking most extensionless paths as chmod +x.
pub static DEFAULT_RULES: sync::LazyLock<Vec<Rule>> = sync::LazyLock::new(|| {
    vec![
        Rule {
            when: Condition {
                mode: None,
                path: None,
            },
            mtime: None,
            uid: None,
            gid: None,
            username: None,
            groupname: None,
            permissions: Some(0o755u32),
        },
        Rule {
            when: Condition {
                mode: Some(FileMode::File),
                path: Some(&*COMMON_NONEXECUTABLE_FILE_PATH_PATTERN),
            },
            mtime: None,
            uid: None,
            gid: None,
            username: None,
            groupname: None,
            permissions: Some(0o644u32),
        },
        Rule {
            when: Condition {
                mode: Some(FileMode::File),
                path: Some(&*EXTENSIONED_FILE_PATH_PATTERN),
            },
            mtime: None,
            uid: None,
            gid: None,
            username: None,
            groupname: None,
            permissions: Some(0o644u32),
        },
        Rule {
            when: Condition {
                mode: None,
                path: Some(&*SYSTEM_V_INIT_LINEAGE_PATTERN),
            },
            mtime: None,
            uid: None,
            gid: None,
            username: None,
            groupname: None,
            permissions: Some(0o755u32),
        },
    ]
});

impl Rule<'_> {
    /// is_match determines whether a rule relates to an entry.
    pub fn is_match(&self, filemode: &FileMode, pth: &str) -> Result<bool, io::Error> {
        if let Some(when_mode) = &self.when.mode
            && when_mode != filemode
        {
            return Ok(false);
        }

        if let Some(when_path) = &self.when.path
            && !when_path.is_match(pth).map_err(io::Error::other)?
        {
            return Ok(false);
        }

        Ok(true)
    }

    /// apply modifies headers.
    pub fn apply(&self, header: &mut tar::Header) -> Result<(), io::Error> {
        if let Some(mtime) = self.mtime {
            header.set_mtime(mtime);
        }

        if let Some(uid) = self.uid {
            header.set_uid(uid);
        }

        if let Some(gid) = self.gid {
            header.set_gid(gid);
        }

        if let Some(username) = &self.username {
            header.set_username(username)?;
        }

        if let Some(groupname) = &self.groupname {
            header.set_groupname(groupname)?;
        }

        if let Some(permissions) = &self.permissions {
            header.set_mode(*permissions);
        }

        Ok(())
    }
}

/// Chandler assembles gunzipped tarballs (TGZ, TAR.GZ).
#[derive(Debug)]
pub struct Chandler<'a> {
    /// verbose enables additional logging.
    pub verbose: bool,

    /// header_type denotes a tape archive format.
    pub header_type: HeaderType,

    /// cwd customizes the current working directory.
    pub cwd: Option<path::PathBuf>,

    /// skip_path_pattern excludes file paths from archival,
    pub skip_path_pattern: Option<&'a fancy_regex::Regex>,

    /// rules collects a table of permissions to apply to inbound files.
    pub rules: Option<&'a Vec<Rule<'a>>>,
}

/// permissions_to_u32 converts fs::Permissions objects to chmod integers.
pub fn permissions_to_u32(permissions: fs::Permissions) -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        permissions.mode()
    }
    #[cfg(windows)]
    {
        if permissions.readonly() {
            0o444u32
        } else {
            0o666u32
        }
    }
}

impl Default for Chandler<'_> {
    /// default constructs an executable-aggressive Chandler configuration.
    fn default() -> Self {
        Chandler {
            verbose: false,
            header_type: HeaderType::UStar,
            cwd: None,
            skip_path_pattern: Some(&*DEFAULT_SKIP_PATH_PATTERN),
            rules: Some(&*DEFAULT_RULES),
        }
    }
}

impl Chandler<'_> {
    /// archive generates a tarball.
    pub fn archive(&self, target: &path::Path, source: &path::Path) -> Result<(), io::Error> {
        let skip_path_pattern: &fancy_regex::Regex = self
            .skip_path_pattern
            .unwrap_or(&*DEFAULT_SKIP_PATH_PATTERN);

        let rules: &Vec<Rule> = self.rules.unwrap_or(&*DEFAULT_RULES);

        if let Some(cwd_pathbuf) = &self.cwd {
            env::set_current_dir(cwd_pathbuf.as_path())?;
        }

        let file = fs::File::create(target)?;
        let gz_encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut builder = tar::Builder::new(gz_encoder);
        let walker = walkdir::WalkDir::new(source).sort_by(
            |a: &walkdir::DirEntry, b: &walkdir::DirEntry| a.file_name().cmp(b.file_name()),
        );

        for entry in walker {
            let entry = entry?;
            let pth = entry.path();
            let pth_clean = pth.normalize();
            let pth_clean_str = pth_clean.to_str().ok_or_else(|| {
                io::Error::other(format!("unable to render path {:?}", pth_clean))
            })?;

            if pth_clean_str.is_empty() || pth_clean_str == "." {
                continue;
            }

            let pth_abs = pth.canonicalize()?;
            let pth_abs_str = pth_abs.to_str().ok_or(io::Error::other(format!(
                "unable to process path: {}",
                pth_abs.display()
            )))?;

            if skip_path_pattern
                .is_match(pth_abs_str)
                .map_err(|e| io::Error::other(e.to_string()))?
            {
                if self.verbose {
                    eprintln!("skipping {pth_clean_str}");
                }

                continue;
            }

            let metadata = entry.metadata()?;
            let mut header = match self.header_type {
                HeaderType::Old => tar::Header::new_old(),
                HeaderType::Gnu => tar::Header::new_gnu(),
                HeaderType::UStar => tar::Header::new_ustar(),
            };

            header.set_path(&pth_clean)?;

            let mtime = metadata
                .modified()?
                .duration_since(time::UNIX_EPOCH)
                .map(|e| e.as_secs())
                .map_err(io::Error::other)?;

            header.set_mtime(mtime);
            header.set_mode(permissions_to_u32(metadata.permissions()));

            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                header.set_uid(metadata.uid() as u64);
                header.set_gid(metadata.gid() as u64);
            }
            #[cfg(not(unix))]
            {
                eprintln!("warning: nonunix environment. dropping uid, gid.");
            }

            let filemode = if metadata.is_dir() {
                FileMode::Directory
            } else if metadata.is_file() {
                FileMode::File
            } else {
                return Err(io::Error::other(format!(
                    "unsupported file type: {pth_clean_str}"
                )));
            };

            if filemode == FileMode::Directory {
                header.set_entry_type(tar::EntryType::Directory);
                header.set_size(0);
            } else if filemode == FileMode::File {
                header.set_size(metadata.len());
            }

            if self.verbose {
                eprintln!("a {pth_clean_str}");
            }

            for rule in rules {
                if !rule.is_match(&filemode, pth_clean_str)? {
                    continue;
                }

                rule.apply(&mut header)?;
            }

            header.set_cksum();

            if filemode == FileMode::Directory {
                builder.append(&header, &[] as &[u8])?;
            } else if filemode == FileMode::File {
                let mut source_file = fs::File::open(pth_clean)?;
                builder.append(&header, &mut source_file)?;
            }
        }

        builder.into_inner()?.finish().map(|_| ())
    }
}
