//! chandler assembles tape archives.

extern crate flate2;
extern crate lazy_static;
extern crate normalize_path;
extern crate regex;
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
use std::time;

lazy_static::lazy_static! {
    /// EXTENSIONED_FILE_PATH_PATTERN matches file paths with extensions,
    /// including file extensions (.BAT, .EXE, and so on),
    /// as well as file paths missing traditional basenames (.gitignore, .git, and so on).
    pub static ref EXTENSIONED_FILE_PATH_PATTERN: regex::Regex = regex::Regex::new(r"^(.*/)*[^/]*\.[^/]*$").unwrap();
}

#[test]
fn test_extensioned_file_path_pattern() {
    let pattern = EXTENSIONED_FILE_PATH_PATTERN.clone();
    assert!(!pattern.is_match("hello"));
    assert!(!pattern.is_match("HELLO"));
    assert!(!pattern.is_match("hello-1.0/docs"));
    assert!(pattern.is_match("hello.bat"));
    assert!(pattern.is_match("HELLO.BAT"));
    assert!(pattern.is_match("hello.exe"));
    assert!(pattern.is_match("HELLO.EXE"));
    assert!(pattern.is_match(".gitignore"));
    assert!(pattern.is_match("degenerate."));
    assert!(pattern.is_match("DEGENERATE."));
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
pub struct Condition {
    /// mode denotes an FileMode.
    pub mode: Option<FileMode>,

    /// path denotes a file path.
    pub path: Option<regex::Regex>,
}

/// Rule applies given permissions for matching file patterns.
#[derive(Debug)]
pub struct Rule {
    /// when denotes a condition required to apply this rule's effects.
    pub when: Condition,

    /// skip excludes archive entries.
    pub skip: bool,

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

impl Rule {
    /// is_match determines whether a rule relates to an entry.
    pub fn is_match(&self, filemode: &FileMode, pth: &str) -> bool {
        if let Some(when_mode) = &self.when.mode
            && when_mode != filemode
        {
            return false;
        }

        if let Some(when_path) = &self.when.path
            && !when_path.is_match(pth)
        {
            return false;
        }

        true
    }

    /// is_skip determines whether a rule skips an entry.
    pub fn is_skip(&self, filemode: &FileMode, pth: &str) -> bool {
        self.is_match(filemode, pth) && self.skip
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
pub struct Chandler {
    /// verbose enables additional logging.
    pub verbose: bool,

    /// header_type denotes a tape archive format.
    pub header_type: HeaderType,

    /// cwd customizes the current working directory.
    pub cwd: Option<path::PathBuf>,

    /// rules collects a table of permissions to apply to inbound files.
    pub rules: Vec<Rule>,
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

impl Default for Chandler {
    /// default constructs an executable-aggressive Chandler configuration.
    fn default() -> Self {
        Chandler {
            verbose: false,
            header_type: HeaderType::UStar,
            cwd: None,
            rules: vec![
                Rule{
                    when: Condition{ mode: None, path: Some(regex::Regex::new(r"^(\.DS_Store)|(Thumbs\.db)$").unwrap()) },
                    skip: true,
                    mtime: None,
                    uid: None,
                    gid: None,
                    username: None,
                    groupname: None,
                    permissions: None,
                },
                Rule{
                    when: Condition{ mode: None, path: None },
                    skip: false,
                    mtime: None,
                    uid: Some(1000u64),
                    gid: Some(1000u64),
                    username: None,
                    groupname: None,
                    permissions: Some(0o755u32),
                },
                Rule{
                    when: Condition{ mode: Some(FileMode::File), path: Some(regex::Regex::new(r"(?i)^bashrc|bsdmakefile|changelog|gnumakefile|license|makefile|readme|aliases|exports|fstab|group|hosts|issue|kshrc|mime|mkshrc|modules|profile|protocols|resolv|services|temp|tmp|zshenv|zshrc|((.*/)?etc/.+)$").unwrap()) },
                    skip: false,
                    mtime: None,
                    uid: None,
                    gid: None,
                    username: None,
                    groupname: None,
                    permissions: Some(0o644u32),
                },
                Rule{
                    when: Condition{ mode: Some(FileMode::File), path: Some(EXTENSIONED_FILE_PATH_PATTERN.clone()) },
                    skip: false,
                    mtime: None,
                    uid: None,
                    gid: None,
                    username: None,
                    groupname: None,
                    permissions: Some(0o644u32),
                },
                Rule{
                    when: Condition{ mode: Some(FileMode::File), path: Some(regex::Regex::new(r"(?i)^(.*/)?etc/init.d/.+$").unwrap()) },
                    skip: false,
                    mtime: None,
                    uid: Some(1000u64),
                    gid: Some(1000u64),
                    username: None,
                    groupname: None,
                    permissions: Some(0o755u32),
                },
            ],
        }
    }
}

impl Chandler {
    /// archive generates a tarball.
    pub fn archive(&self, target: &path::Path, source: &path::Path) -> Result<(), io::Error> {
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
            let pth_str = pth_clean.to_str().ok_or_else(|| {
                io::Error::other(format!("unable to render path {:?}", pth_clean))
            })?;

            if pth_str.is_empty() || pth_str == "." {
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

            let filemode = if metadata.is_dir() {
                FileMode::Directory
            } else if metadata.is_file() {
                FileMode::File
            } else {
                return Err(io::Error::other(format!(
                    "unsupported file type: {pth_str}"
                )));
            };

            if filemode == FileMode::Directory {
                header.set_entry_type(tar::EntryType::Directory);
                header.set_size(0);
            } else if filemode == FileMode::File {
                header.set_size(metadata.len());
            }

            if self.verbose {
                eprintln!("a {pth_str}");
            }

            if self.rules.iter().any(|e| e.is_skip(&filemode, pth_str)) {
                continue;
            }

            for rule in &self.rules {
                if !rule.is_match(&filemode, pth_str) {
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
