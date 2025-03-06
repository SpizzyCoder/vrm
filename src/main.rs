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
    paths: Vec<String>,
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

    if args.paths.len() < 1 {
        printer.stderr(format!["Must have at least one argument"]);
        return;
    }

    for cur_path in args.paths.iter() {
        if !Path::new(cur_path).exists() {
            printer.stderr(format!["{} does not exist", cur_path]);
            errors += 1;
            continue;
        }

        if Path::new(cur_path).is_file() {
            printer.verbose(format!["Deleting: {}", cur_path]);

            if let Err(err) = fs::remove_file(cur_path) {
                printer.stderr(format!["Failed to delete file {}: {}", cur_path, err]);
                errors += 1;
            } else {
                deleted_files += 1;
            }
        } else if Path::new(cur_path).is_dir() {
            let walk_dir: WalkDir = WalkDir::new(cur_path)
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
                    errors += 1;
                }
            }
        } else {
            printer.stderr(format!["Unsupported type {}", cur_path]);
            errors += 1;
        }
    }

    if args.verbose {
        printer.stdout(format!["=========="]);
    }

    if deleted_files != 0 {
        printer.stdout(format!["Deleted files: {}", deleted_files]);
    }

    if deleted_directories != 0 {
        printer.stdout(format!["Deleted directories: {}", deleted_directories]);
    }

    if errors != 0 {
        printer.stdout(format!["Errors: {}", errors]);
    }
}
