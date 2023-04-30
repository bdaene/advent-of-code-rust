use core::panic;
use itertools::Itertools;

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
pub enum Terminal {
    ChangeDirectory(String),
    ListDirectory,
    Directory(String),
    File(usize, String),
}

#[derive(PartialEq, Debug)]
pub struct Solution {
    terminal: Vec<Terminal>,
}

type ContentIndex = usize;

struct Directory {
    name: String,
    content: Vec<ContentIndex>,
}

struct File {
    _name: String,
    size: usize,
}

enum Content {
    File(File),
    Directory(Directory),
}

fn get_current_directory(current_directory: ContentIndex, contents: &Vec<Content>) -> &Directory {
    match &contents[current_directory] {
        Content::Directory(directory) => directory,
        Content::File(_) => panic!("Current directory should be a directory."),
    }
}

fn get_current_directory_mut(
    current_directory: ContentIndex,
    contents: &mut Vec<Content>,
) -> &mut Directory {
    match &mut contents[current_directory] {
        Content::Directory(directory) => directory,
        Content::File(_) => panic!("Current directory should be a directory."),
    }
}

fn build_content_tree(terminal: &Vec<Terminal>) -> Vec<Content> {
    let mut contents = vec![Content::Directory(Directory {
        name: "root".to_owned(),
        content: vec![],
    })];
    let root = 0_usize;
    let mut current_path = vec![];
    let mut current_directory = root;

    for entry in terminal.iter() {
        match entry {
            Terminal::ChangeDirectory(path) => {
                match path.as_str() {
                    "/" => {
                        current_path = vec![];
                        current_directory = root;
                    }
                    ".." => {
                        current_directory = current_path.pop().expect("Cannot cd above root.");
                    }
                    path => {
                        let current_content =
                            &get_current_directory(current_directory, &contents).content;
                        let new_directory = current_content
                            .iter()
                            .find(|index| {
                                if let Content::Directory(Directory { name, .. }) =
                                    &contents[**index]
                                {
                                    name == path
                                } else {
                                    false
                                }
                            })
                            .expect("{path} not found in current directory")
                            .clone();

                        current_path.push(current_directory);
                        current_directory = new_directory;
                    }
                };
            }
            Terminal::ListDirectory => (),
            Terminal::Directory(name) => {
                let directory = contents.len();
                contents.push(Content::Directory(Directory {
                    name: name.clone(),
                    content: Vec::new(),
                }));
                get_current_directory_mut(current_directory, &mut contents)
                    .content
                    .push(directory);
            }
            Terminal::File(size, name) => {
                let file = contents.len();
                contents.push(Content::File(File {
                    _name: name.clone(),
                    size: *size,
                }));
                get_current_directory_mut(current_directory, &mut contents)
                    .content
                    .push(file);
            }
        }
    }
    contents
}

fn compute_directory_sizes(contents: &Vec<Content>) -> Vec<usize> {
    let mut sizes = vec![None; contents.len()];

    for (i, content) in contents.iter().enumerate().rev() {
        sizes[i] = Some(match content {
            Content::File(File { size, .. }) => *size,
            Content::Directory(Directory { content, .. }) => content
                .iter()
                .map(|index| sizes[*index].expect("Should have been computed before."))
                .sum::<usize>(),
        })
    }

    sizes
        .iter()
        .zip(contents.iter())
        .filter_map(|(size, content)| match content {
            Content::Directory(_) => Some(size.expect("Should have been computed.")),
            Content::File(_) => None,
        })
        .collect()
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let terminal = data
            .lines()
            .map(|entry| {
                if let Some(directory) = entry.strip_prefix("$ cd ") {
                    Terminal::ChangeDirectory(directory.to_owned())
                } else if entry == "$ ls" {
                    Terminal::ListDirectory
                } else if let Some(directory) = entry.strip_prefix("dir ") {
                    Terminal::Directory(directory.to_owned())
                } else {
                    let (size, name) = entry
                        .split(" ")
                        .collect_tuple()
                        .expect("File size and name.");
                    Terminal::File(
                        size.parse().expect("Size should be a number."),
                        name.to_owned(),
                    )
                }
            })
            .collect();

        Solution { terminal: terminal }
    }

    fn part_1(&self) -> String {
        const SIZE_LIMIT: usize = 100_000;

        let contents = build_content_tree(&self.terminal);
        let directory_sizes = compute_directory_sizes(&contents);

        directory_sizes
            .iter()
            .filter(|size| **size <= SIZE_LIMIT)
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        const TOTAL_SIZE: usize = 70_000_000;
        const NEEDED_SIZE: usize = 30_000_000;

        let contents = build_content_tree(&self.terminal);
        let directory_sizes = compute_directory_sizes(&contents);

        let used_space = directory_sizes[0];

        directory_sizes
            .iter()
            .filter(|size| TOTAL_SIZE - used_space + **size >= NEEDED_SIZE)
            .min()
            .expect("At least root is big enough.")
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data = fs::read_to_string("data/day_07_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                terminal: vec![
                    Terminal::ChangeDirectory("/".to_string()),
                    Terminal::ListDirectory,
                    Terminal::Directory("a".to_string()),
                    Terminal::File(14848514, "b.txt".to_string()),
                    Terminal::File(8504156, "c.dat".to_string()),
                    Terminal::Directory("d".to_string()),
                    Terminal::ChangeDirectory("a".to_string()),
                    Terminal::ListDirectory,
                    Terminal::Directory("e".to_string()),
                    Terminal::File(29116, "f".to_string()),
                    Terminal::File(2557, "g".to_string()),
                    Terminal::File(62596, "h.lst".to_string()),
                    Terminal::ChangeDirectory("e".to_string()),
                    Terminal::ListDirectory,
                    Terminal::File(584, "i".to_string()),
                    Terminal::ChangeDirectory("..".to_string()),
                    Terminal::ChangeDirectory("..".to_string()),
                    Terminal::ChangeDirectory("d".to_string()),
                    Terminal::ListDirectory,
                    Terminal::File(4060174, "j".to_string()),
                    Terminal::File(8033020, "d.log".to_string()),
                    Terminal::File(5626152, "d.ext".to_string()),
                    Terminal::File(7214296, "k".to_string())
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "95437");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "24933642");
    }
}
