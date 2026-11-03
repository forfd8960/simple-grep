use std::{fs::File, io::Read, path::PathBuf};

use regex::bytes::{Regex, RegexBuilder};

use crate::{Args, GrepResult};

#[derive(Debug)]
pub struct FileResult {
    pub path: PathBuf,
    pub lines: Vec<Vec<u8>>,
    pub matched: Vec<usize>,
}

#[derive(Debug)]
pub struct Matcher {
    pub reg: Regex,
}

impl Matcher {
    pub fn new(args: &Args) -> GrepResult<Self> {
        let patterns = args.all_patterns();
        let joined = patterns
            .iter()
            .map(|pat| format!("(?:{pat})"))
            .collect::<Vec<_>>()
            .join("|");

        let reg = RegexBuilder::new(&joined).build()?;
        Ok(Self { reg })
    }

    pub fn is_match(&self, line: &[u8]) -> bool {
        self.reg.is_match(line)
    }
}

pub fn split_lines(data: &[u8]) -> Vec<Vec<u8>> {
    data.split(|e| *e == '\n' as u8)
        .into_iter()
        .map(|d| d.to_vec())
        .collect()
}

pub fn search_file(path: &PathBuf, matcher: &Matcher) -> GrepResult<Option<FileResult>> {
    let mut data = Vec::new();
    File::open(path)?.read_to_end(&mut data)?;

    if data.contains(&0) {
        return Ok(None);
    }

    let lines = split_lines(&data);
    let mut matched = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if matcher.is_match(line) {
            matched.push(index + 1);
        }
    }

    Ok(Some(FileResult {
        path: path.clone(),
        lines,
        matched,
    }))
}

#[cfg(test)]
mod tests {
    use crate::search::split_lines;

    #[test]
    fn test_pattern_format() {
        let patterns = ["hello", "rust"];
        let joined = patterns
            .iter()
            .map(|pattern| format!("(?:{pattern})"))
            .collect::<Vec<_>>()
            .join("|");
        println!("{}", joined);
    }

    #[test]
    fn test_split_lines() {
        let lines = "hello\nworld\nnice to see you".as_bytes();
        let split_lines = split_lines(lines);
        println!("{:?}", split_lines);

        let res = vec![
            "hello".as_bytes(),
            "world".as_bytes(),
            "nice to see you".as_bytes(),
        ];
        println!("{:?}", res);
        assert_eq!(split_lines, res);
    }
}
