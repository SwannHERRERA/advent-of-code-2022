use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub type FolderContent = Vec<ElveFile>;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    List,
    Cd(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElveFile {
    File(File),
    Folder(Rc<RefCell<Folder>>),
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub size: Option<usize>,
    pub files: FolderContent,
    pub path: String,
}

impl Default for Folder {
    fn default() -> Self {
        Folder {
            size: None,
            files: vec![],
            path: f("/"),
        }
    }
}

impl PartialEq for Folder {
    fn eq(&self, other: &Self) -> bool {
        if self.path != other.path {
            return false;
        }
        if self.size != other.size {
            return false;
        }
        if self.files.len() != other.files.len() {
            return false;
        }
        for (mine, other) in self.files.iter().zip(other.files.iter()) {
            if *mine != *other {
                return false;
            }
        }
        true
    }
}

impl Folder {
    pub fn new<S: Into<String>>(files: FolderContent, path: S) -> Self {
        Folder {
            size: None,
            files,
            path: path.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub size: usize,
    pub path: String,
}

impl File {
    pub fn new<S: Into<String>>(size: usize, path: S) -> Self {
        File {
            size,
            path: path.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_equal() {
        let folder_1 = Folder::new(vec![], "ABC");
        let folder_2 = Folder::new(vec![], "BCA");
        println!("{:?}", folder_1);
        println!("{:?}", folder_2);
        assert_ne!(folder_1, folder_2);

        let mut folder_1 = Folder::new(vec![], "ABC");
        let mut folder_2 = Folder::new(vec![], "ABC");
        println!("{:?}", folder_1);
        println!("{:?}", folder_2);
        assert_eq!(folder_1, folder_2);

        folder_1.size = Some(799);
        assert_ne!(folder_1, folder_2);

        folder_2.size = Some(8);
        assert_ne!(folder_1, folder_2);

        folder_2.size = Some(799);
        assert_eq!(folder_1, folder_2);

        folder_1.files.push(ElveFile::File(File::new(40, "ABC/x")));
        assert_ne!(folder_1, folder_2);

        folder_2.files.push(ElveFile::File(File::new(40, "ABC/y")));
        assert_ne!(folder_1, folder_2);

        folder_2.files.pop();
        folder_2.files.push(ElveFile::File(File::new(40, "ABC/x")));
        assert_eq!(folder_1, folder_2);
    }
}
