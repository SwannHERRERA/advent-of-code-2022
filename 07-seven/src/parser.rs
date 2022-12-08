use std::cell::RefCell;
use std::rc::Rc;

use crate::builder::ElveFileSystemBuilder;
use crate::prelude::*;
use crate::types::{Command, ElveFile, File};

pub struct Parser {
    builder: ElveFileSystemBuilder,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            builder: ElveFileSystemBuilder::new(),
        }
    }

    pub fn parse_commands(&mut self, input: &str) -> Result<ElveFile> {
        for line in input.lines().skip(1) {
            // println!("{}", line);
            if line.starts_with('$') {
                let command = self.parse_command(line)?;
                match command {
                    Command::List => {} // do nothing
                    Command::Cd(path) => {
                        self.builder.moves(&path);
                    }
                }
            } else if let Ok(file) = self.parse_file_line(line) {
                self.builder.add_file(ElveFile::File(file));
            }
        }
        Ok(ElveFile::Folder(Rc::new(RefCell::new(
            self.builder.build(),
        ))))
    }

    fn parse_command(&self, input: &str) -> Result<Command> {
        if input.starts_with("$ ls") {
            return Ok(Command::List);
        }
        if input.starts_with("$ cd") {
            if let Some((_, path)) = input.split_once("$ cd ") {
                return Ok(Command::Cd(path.to_string()));
            }
        }
        Err(Error::Generic(f("failed to parse command")))
    }

    fn parse_file_line(&self, line: &str) -> Result<File> {
        let Some((size, name)) = line.split_once(' ') else {
            return Err(Error::Generic(f("no space on this line (parse file)")));
        };
        if size == "dir" {
            return Err(Error::Generic(f("it's a dir")));
        }
        let size: usize = size.parse().unwrap();
        if self.builder.current_path == "/" {
            return Ok(File::new(size, format!("/{}", name)));
        }
        Ok(File::new(
            size,
            format!("{}/{}", self.builder.current_path, name),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_parse_command_line() {
        const COMMAND_LINE: &str = "$ ls";
        const EXPECTED: Command = Command::List;
        let parser = Parser::new();
        if let Ok(res) = parser.parse_command(COMMAND_LINE) {
            assert_eq!(EXPECTED, res);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_cd() {
        const COMMAND_LINE: &str = "$ cd a";
        let expected: Command = Command::Cd(f("a"));
        let parser = Parser::new();
        if let Ok(res) = parser.parse_command(COMMAND_LINE) {
            assert_eq!(expected, res);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_file_line() {
        let parser = Parser::new();
        let expected = File::new(4060174, "/j.exe");
        const LINE: &str = "4060174 j.exe";
        if let Ok(res) = parser.parse_file_line(LINE) {
            assert_eq!(expected, res);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_commands() {
        let mut builder = ElveFileSystemBuilder::new();
        builder
            .add_file(ElveFile::File(File::new(14848514, "/b.txt")))
            .add_file(ElveFile::File(File::new(8504156, "/c.dat")))
            .moves("a")
            .add_file(ElveFile::File(File::new(29116, "/a/f")))
            .add_file(ElveFile::File(File::new(2557, "/a/g")))
            .add_file(ElveFile::File(File::new(62596, "/a/h.lst")))
            .moves("e")
            .add_file(ElveFile::File(File::new(584, "/a/e/i")))
            .moves("..")
            .moves("..")
            .moves("d")
            .add_file(ElveFile::File(File::new(4060174, "/d/j")))
            .add_file(ElveFile::File(File::new(8033020, "/d/d.log")))
            .add_file(ElveFile::File(File::new(5626152, "/d/d.ext")))
            .add_file(ElveFile::File(File::new(7214296, "/d/k")));

        let expected = builder.build();

        let mut parser = Parser::new();
        let res = parser.parse_commands(INPUT).unwrap();

        match res {
            ElveFile::File(_) => unreachable!(),
            ElveFile::Folder(folder) => {
                assert_eq!(expected, folder.take());
            }
        }
    }
}
