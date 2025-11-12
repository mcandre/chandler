//! CLI chandler tool

extern crate chandler;
extern crate die;
extern crate getopts;

use die::{Die, die};
use std::env;
use std::path;

/// CLI entrypoint
fn main() {
    let brief: String = format!(
        "Usage: {} <OPTIONS> <source directory>",
        env!("CARGO_PKG_NAME")
    );

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optopt("C", "cwd", "customize current working directory", "<dir>");
    opts.optflag("v", "verbose", "enable additional logging");
    opts.optopt("f", "file", "output path (TGZ or TAR.GZ)", "<archive>");
    opts.optflag("h", "help", "print usage info");
    opts.optflag("V", "version", "print version info");

    let usage: String = opts.usage(&brief);
    let arguments: Vec<String> = env::args().collect();
    let optmatches: getopts::Matches = opts.parse(&arguments[1..]).die(&usage);

    if optmatches.opt_present("h") {
        die!(0; usage);
    }

    if optmatches.opt_present("V") {
        die!(0; format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    }

    let mut ch = chandler::Chandler::default();

    if optmatches.opt_present("v") {
        ch.verbose = true;
    }

    if optmatches.opt_present("C") {
        let cwd_string = optmatches.opt_str("C").die(&usage);
        ch.cwd = Some(path::PathBuf::from(cwd_string))
    }

    let archive_string = optmatches.opt_str("f").die(&usage);
    let archive_path: &path::Path = path::Path::new(&archive_string);
    let mut final_archive = archive_path.to_path_buf();

    if let Some(cwd) = &ch.cwd {
        final_archive = cwd.join(archive_path);
    }

    let final_archive_string = match final_archive.to_str() {
        Some(v) => v,
        None => die!(format!("unable to render path {:?}", final_archive)),
    };

    let args = optmatches.free;

    if args.len() != 1 {
        die!(1; usage);
    }

    match ch.archive(archive_path, path::Path::new(&args[0])) {
        Err(e) => die!(e.to_string()),
        _ => eprintln!("archived entries to {final_archive_string}"),
    }
}
