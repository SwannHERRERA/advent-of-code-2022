use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;
use crate::types::{ElveFile, Folder};

#[derive(Debug, Clone)]
pub struct ElveFileSystemBuilder {
    root: Rc<RefCell<Folder>>,
    pub current_path: String,
}

impl ElveFileSystemBuilder {
    pub fn new() -> Self {
        ElveFileSystemBuilder {
            root: Rc::new(RefCell::new(Folder::new(Vec::new(), f("/")))),
            current_path: f("/"),
        }
    }

    pub fn add_file(&mut self, file: ElveFile) -> &mut Self {
        let searched_path: String = self.current_path.clone();
        let dest = self.find_folder(searched_path.as_str());
        match dest {
            Ok(dest) => {
                let mut dest = dest.borrow_mut();
                // println!("{:?}", file);
                dest.files.push(file);
            }
            Err(err) => println!("{:?}", err),
        }
        self
    }

    pub fn moves(&mut self, path: &str) -> &mut Self {
        // println!("move path: {}", path);
        if path == ".." {
            let mut segments: Vec<&str> = self.current_path.split('/').collect();
            segments.pop();
            self.current_path = segments.join("/");
            if self.current_path.is_empty() {
                self.current_path = "/".to_string();
            }
            return self;
        }
        let mut new_folder_path = format!("{}/{}", self.current_path, path);
        if self.current_path == "/" {
            new_folder_path = format!("/{}", path);
        }
        let new_folder = Folder::new(vec![], new_folder_path.clone());
        self.add_file(ElveFile::Folder(Rc::new(RefCell::new(new_folder))));
        self.current_path = new_folder_path;
        self
    }

    pub fn build(&mut self) -> Folder {
        compute_sizes(self.root.clone());
        (*self.root).take()
    }

    fn find_folder(&mut self, search_path: &str) -> Result<Rc<RefCell<Folder>>> {
        find_folder_recu(self.root.clone(), search_path)
    }
}

fn compute_sizes(node: Rc<RefCell<Folder>>) -> usize {
    let mut borrow = node.borrow_mut();
    let sum = borrow
        .files
        .iter()
        .map(|file| match file {
            ElveFile::File(file) => file.size,
            ElveFile::Folder(folder) => compute_sizes(folder.clone()),
        })
        .sum::<usize>();
    let _ = borrow.size.insert(sum);
    sum
}

fn find_folder_recu(folder: Rc<RefCell<Folder>>, search_path: &str) -> Result<Rc<RefCell<Folder>>> {
    let folder_copy = folder.borrow();
    if folder_copy.path == search_path {
        return Ok(folder.clone());
    }
    for item in &folder_copy.files {
        match item {
            ElveFile::File(_) => continue,
            ElveFile::Folder(sub_folder) => {
                let res = find_folder_recu(sub_folder.clone(), search_path);
                if res.is_ok() {
                    return res;
                }
            }
        }
    }
    Err(Error::Generic(format!("folder not found {}", search_path)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::File;

    #[test]
    fn test_add_file_to_root() {
        let mut expected_folder = Folder::new(vec![ElveFile::File(File::new(584, "i"))], f("/"));
        expected_folder.size = Some(584);

        let mut builder = ElveFileSystemBuilder::new();
        let file = File::new(584, "i");
        let folder = builder.add_file(ElveFile::File(file)).build();
        assert_eq!(expected_folder, folder);
    }

    #[test]
    fn test_add_folder_to_root() {
        let folder = Rc::new(RefCell::new(Folder::new(vec![], "a")));
        let mut expected_folder = Folder::new(vec![ElveFile::Folder(folder.clone())], f("/"));
        expected_folder.size = Some(0);

        let mut builder = ElveFileSystemBuilder::new();
        let res_folder = builder.add_file(ElveFile::Folder(folder.clone())).build();
        assert_eq!(expected_folder, res_folder);
    }

    #[test]
    fn test_with_complexe_struct() {
        let file1 = ElveFile::File(File::new(311, "x"));
        let file2 = ElveFile::File(File::new(200, "y"));
        let file3 = ElveFile::File(File::new(1, "z"));
        let file4 = ElveFile::File(File::new(31, "1"));

        let folder_1 = Rc::new(RefCell::new(Folder::new(vec![file2], "/a")));
        let folder_3 = Rc::new(RefCell::new(Folder::new(vec![file3, file4], "/b/c")));
        let folder_2 = Rc::new(RefCell::new(Folder::new(
            vec![file1, ElveFile::Folder(folder_3)],
            "/b",
        )));
        let mut expected_folder = Folder::new(
            vec![
                ElveFile::Folder(folder_1.clone()),
                ElveFile::Folder(folder_2.clone()),
            ],
            f("/"),
        );
        expected_folder.size = Some(311 + 200 + 1 + 31);

        let mut builder = ElveFileSystemBuilder::new();
        let res_folder = builder
            .add_file(ElveFile::Folder(folder_1.clone()))
            .add_file(ElveFile::Folder(folder_2.clone()))
            .build();
        assert_eq!(expected_folder, res_folder);
    }

    #[test]
    fn test_moves() {
        let mut builder = ElveFileSystemBuilder::new();
        builder
            .moves("abc")
            .moves("gac")
            .moves("..")
            .moves("targamas");
        let root = builder.build();

        assert_eq!(1, root.files.len());

        let abc = &root.files[0];
        let abc_len = match abc {
            ElveFile::File(_) => unreachable!(),
            ElveFile::Folder(folder) => folder.borrow().files.len(),
        };
        assert_eq!(2, abc_len);
    }
}
