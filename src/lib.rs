pub mod cmd;
pub mod search;
pub mod walk;

use std::io;

use thiserror::Error;

pub use cmd::Args;
pub use search::{Matcher, search_file};
pub use walk::collect_files;

#[derive(Debug, Error)]
pub enum GrepErrors {
    #[error("io error: {0}")]
    IOError(#[from] io::Error),

    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("command error: {0}")]
    CmdError(String),
    #[error("ignore error: {0}")]
    IgnoreError(#[from] ignore::Error),
}

pub type GrepResult<T> = Result<T, GrepErrors>;
