use std::process::ExitCode;

use clap::Parser;
use simple_grep::{Args, GrepErrors, GrepResult, Matcher, search::search_file};

fn main() -> ExitCode {
    println!("A mini grep implement with Rust");
    let args = Args::parse();
    println!("{args:#?}");

    match run(&args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("match failed: {e}");
            ExitCode::from(2)
        }
    }
}

pub fn run(args: &Args) -> GrepResult<ExitCode> {
    let matcher = Matcher::new(args)?;
    let path = args
        .all_paths()
        .first()
        .cloned()
        .ok_or(GrepErrors::CmdError(format!("no path")))?;

    println!("path: {:?}", path);

    if let Some(file_result) = search_file(&path, matcher)? {
        for line in file_result.matched {
            println!(
                "{}: {}",
                line,
                String::from_utf8_lossy(&file_result.lines[line - 1])
            );
        }
        Ok(ExitCode::SUCCESS)
    } else {
        println!("search has no result");
        Ok(ExitCode::SUCCESS)
    }
}
