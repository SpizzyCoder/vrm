use crate::Args;

pub struct Printer {
    verbose: bool,
    quiet: bool,
}

impl Printer {
    pub fn new(args: &Args) -> Self {
        return Self {
            verbose: args.verbose,
            quiet: args.quiet,
        };
    }

    pub fn stdout(&self, message: String) {
        if !self.quiet {
            println!["{}", message];
        }
    }

    pub fn stderr(&self, message: String) {
        eprintln!["{}", message];
    }

    pub fn verbose(&self, message: String) {
        if self.verbose {
            self.stdout(message);
        }
    }
}
