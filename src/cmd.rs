use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "minigrep", version, about = "search file content recursively")]
pub struct Args {
    #[arg(required_unless_present = "regexps", value_name = "PATTERN")]
    pub pattern: Option<String>,

    #[arg(short = 'e', long = "regexp", value_name = "PATTERN")]
    pub regexps: Vec<String>,

    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,
}

impl Args {
    pub fn all_patterns(&self) -> Vec<String> {
        let mut pats = self.regexps.clone();
        if self.regexps.is_empty() {
            if let Some(p) = &self.pattern {
                pats.push(p.clone());
            }
        }
        pats
    }

    pub fn all_paths(&self) -> Vec<PathBuf> {
        if self.regexps.is_empty() {
            return self.paths.clone();
        }

        let mut paths = self.paths.clone();
        if let Some(p) = &self.pattern {
            paths.insert(0, PathBuf::from(p));
        }
        paths
    }
}
