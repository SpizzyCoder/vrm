use std::{fs, path::Path};

use clap::Parser;
use walkdir::{DirEntry, WalkDir};

mod printer;
use printer::Printer;

/// Simple and cross platform cli file/directory deletion tool.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Verbose
    #[arg(short, long)]
    verbose: bool,

    /// Quiet
    #[arg(short, long)]
    quiet: bool,

    /// Path
    #[arg()]
    path: String,
}

fn main() {
    let args: Args = Args::parse();
    let mut deleted_files: u64 = 0;
    let mut deleted_directories: u64 = 0;
    let mut errors: u64 = 0;

    if args.verbose && args.quiet {
        eprintln!["Can't have both --verbose and --quiet"];
        return;
    }

    let printer: Printer = Printer::new(&args);

    if !Path::new(&args.path).exists() {
        printer.stderr(format!["{} does not exist", args.path]);
        return;
    }

    if Path::new(&args.path).is_file() {
        printer.verbose(format!["Deleting: {}", args.path]);

        if let Err(err) = fs::remove_file(&args.path) {
            printer.stderr(format!["Failed to delete {}: {}", args.path, err]);
            errors += 1;
        } else {
            deleted_files += 1;
        }
    } else if Path::new(&args.path).is_dir() {
        let walk_dir: WalkDir = WalkDir::new(&args.path)
            .contents_first(true)
            .follow_links(false)
            .follow_root_links(false);

        for entry_res in walk_dir.into_iter() {
            let unwrapped_entry: DirEntry = match entry_res {
                Ok(entry) => entry,
                Err(err) => {
                    printer.stderr(format!["Failed to get entry: {}", err]);
                    errors += 1;
                    continue;
                }
            };

            printer.verbose(format!["Deleting: {}", unwrapped_entry.path().display()]);
            if unwrapped_entry.path().is_dir() {
                if let Err(err) = fs::remove_dir(unwrapped_entry.path()) {
                    printer.stderr(format![
                        "Failed to delete dir {}: {}",
                        unwrapped_entry.path().display(),
                        err
                    ]);
                    errors += 1;
                } else {
                    deleted_directories += 1;
                }
            } else if unwrapped_entry.path().is_file() {
                if let Err(err) = fs::remove_file(unwrapped_entry.path()) {
                    printer.stderr(format![
                        "Failed to delete file {}: {}",
                        unwrapped_entry.path().display(),
                        err
                    ]);
                    errors += 1;
                } else {
                    deleted_files += 1;
                }
            } else {
                printer.stderr(format![
                    "Unsupported type {}",
                    unwrapped_entry.path().display()
                ]);
            }
        }
    } else {
        printer.stderr(format!["Unsupported type {}", &args.path]);
    }

    if args.verbose {
        printer.stdout(format!["=========="]);
    }
    printer.stdout(format!["Deleted files: {}", deleted_files]);
    printer.stdout(format!["Deleted directories: {}", deleted_directories]);
    printer.stdout(format!["Errors: {}", errors]);
}
