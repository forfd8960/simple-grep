use std::path::PathBuf;

use ignore::WalkBuilder;

use crate::GrepResult;

pub fn collect_files(paths: &[PathBuf]) -> GrepResult<Vec<PathBuf>> {
    let mut roots = paths.to_vec();
    if roots.is_empty() {
        roots.push(PathBuf::from("."));
    }

    let mut builder = WalkBuilder::new(&roots[0]);
    for root in &roots[1..] {
        builder.add(root);
    }

    let mut file_paths = Vec::new();
    for entry in builder.build() {
        let entry = entry?;
        if entry.file_type().is_some_and(|f| f.is_file()) {
            file_paths.push(entry.into_path());
        }
    }

    Ok(file_paths)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::walk::collect_files;

    #[test]
    fn test_walk_files() {
        let paths = [PathBuf::from("src")];
        let files = collect_files(&paths);

        match files {
            Ok(mut fps) => {
                fps.sort();
                let expect = [
                    "src/cmd.rs",
                    "src/lib.rs",
                    "src/main.rs",
                    "src/search.rs",
                    "src/walk.rs",
                ]
                .iter()
                .map(|v| PathBuf::from(v))
                .collect::<Vec<_>>();
                assert_eq!(fps, expect);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
