pub mod cmd;
pub mod search;

use std::io;

use thiserror::Error;

pub use cmd::Args;
pub use search::Matcher;

#[derive(Debug, Error)]
pub enum GrepErrors {
    #[error("io error: {0}")]
    IOError(#[from] io::Error),

    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("command error: {0}")]
    CmdError(String),
}

pub type GrepResult<T> = Result<T, GrepErrors>;
