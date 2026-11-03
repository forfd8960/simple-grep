use std::process::ExitCode;

use clap::Parser;
use colored::Colorize;
use simple_grep::{Args, GrepResult, Matcher, collect_files, search_file};

fn main() -> ExitCode {
    println!("A mini grep implement with Rust");
    let args = Args::parse();
    // println!("{args:#?}");

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
    let paths = collect_files(&args.all_paths())?;

    for path in paths {
        if let Some(file_result) = search_file(&path, &matcher)? {
            if file_result.matched.is_empty() {
                continue;
            }

            println!("{}", path.to_str().unwrap().italic().yellow());
            for line in file_result.matched {
                let line_text = String::from_utf8_lossy(&file_result.lines[line - 1]).bold();
                println!("\t{}: {}", line.to_string().green(), line_text);
            }
        } else {
            println!("search has no result");
            continue;
        }
    }

    Ok(ExitCode::SUCCESS)
}
