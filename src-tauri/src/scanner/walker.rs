use std::{fs::DirEntry, path::{Path, PathBuf}};

pub struct DirWalker {
    path: PathBuf,
    ignore_glob: Vec<String>,
}
impl DirWalker {
    pub fn new<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            ignore_glob: vec![],
        }
    }
}

impl Iterator for DirWalker {
    type Item = Result<DirEntry, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

