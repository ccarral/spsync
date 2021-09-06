use rusqlite::Connection;
use std::error::Error;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub struct FileSystemTree {
    base: PathBuf,
}

impl FileSystemTree {
    pub fn new<P: AsRef<Path>>(base: P, conn: Connection) -> Result<Self, Box<dyn Error>> {
        let base = base.as_ref().to_path_buf();

        if !base.exists() {
            return Err(PathError::NonExistant(base).into());
        }

        if !base.is_dir() {
            return Err(PathError::PathNotDir(base).into());
        }

        Ok(FileSystemTree { base })
    }
}

#[derive(Error, Debug)]
enum PathError {
    #[error("Path does not exist: `{0}`")]
    NonExistant(PathBuf),
    #[error("Path is not a directory: `{0}`")]
    PathNotDir(PathBuf),
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_errors() {
        let conn = Connection::open_in_memory().unwrap();
        let fs_tree_doesnt_exists = FileSystemTree::new("hola", conn);
        assert!(fs_tree_doesnt_exists.is_err());

        let file = if cfg!(target_os = "linux") {
            Path::new("/bin/ls")
        } else {
            panic!("Tests for windows not implemented yet");
        };

        let conn = Connection::open_in_memory().unwrap();

        let file_fstree = FileSystemTree::new(file, conn);

        assert!(file_fstree.is_err());
    }
}
