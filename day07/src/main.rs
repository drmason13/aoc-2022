use std::{
    collections::HashMap,
    fmt,
    marker::PhantomData,
    str::FromStr,
    sync::{mpsc, Arc},
};

use shared::{receive_answers, run_part_threaded, ValueError};

#[derive(Debug)]
struct Arena<T> {
    data: Vec<T>,
}

struct ArenaIter<'a, T> {
    index: usize,
    arena: &'a Arena<T>,
}

impl<'a, T> Iterator for ArenaIter<'a, T> {
    type Item = Index<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.arena.data.len() {
            return None;
        }
        let index = Index {
            value: self.index,
            type_marker: PhantomData,
        };
        self.index += 1;
        Some(index)
    }
}

impl<T> Arena<T> {
    fn new() -> Self {
        Arena { data: Vec::new() }
    }

    fn iter(&self) -> ArenaIter<T> {
        ArenaIter {
            arena: self,
            index: 0,
        }
    }

    fn add(&mut self, item: T) -> Index<T> {
        let index = Index {
            value: self.data.len(),
            type_marker: PhantomData,
        };
        self.data.push(item);
        index
    }

    fn get_mut(&mut self, index: &Index<T>) -> Option<&mut T> {
        self.data.get_mut(index.value)
    }

    fn get(&self, index: &Index<T>) -> Option<&T> {
        self.data.get(index.value)
    }
}

#[derive(Clone)]
struct Index<T> {
    value: usize,
    type_marker: PhantomData<T>,
}

impl<T: fmt::Debug> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Index").field("value", &self.value).finish()
    }
}

#[derive(Debug, Clone)]
struct Directory {
    parent: Option<Index<Directory>>,
    children: HashMap<String, Index<Directory>>,
    files: HashMap<String, Index<File>>,
}

impl Directory {
    fn new(parent: Option<Index<Directory>>) -> Self {
        Directory {
            parent,
            children: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn get_child(&self, name: &str) -> Option<Index<Directory>> {
        self.children.get(name).cloned()
    }

    fn add_new_directory(
        parent_dir: &Index<Directory>,
        arena: &mut Arena<Directory>,
        name: String,
    ) {
        let child_dir = arena.add(Directory::new(Some(parent_dir.clone())));
        arena
            .get_mut(parent_dir)
            .expect("parent directory exists")
            .children
            .insert(name, child_dir);
    }

    fn add_new_file(&mut self, arena: &mut Arena<File>, size: u32, name: String) {
        let new_file = File::new(size);
        let index = arena.add(new_file);
        self.files.insert(name, index);
    }
}

#[derive(Debug, Clone)]
struct File {
    size: u32,
}

impl File {
    fn new(size: u32) -> Self {
        File { size }
    }
}

#[derive(Debug)]
struct FileSystem {
    directories: Arena<Directory>,
    files: Arena<File>,
    root: Index<Directory>,
}

impl FileSystem {
    fn new(directories: Arena<Directory>, files: Arena<File>) -> Self {
        FileSystem {
            directories,
            files,
            root: Index {
                value: 0,
                type_marker: PhantomData,
            },
        }
    }

    fn size(&self, running_size: u32, directory: &Index<Directory>) -> u32 {
        let dir = self.directories.get(directory).expect("dir to size exists");

        let mut size: u32 = dir
            .files
            .values()
            .map(|index| self.files.get(index).expect("file exists").size)
            .sum::<u32>()
            + running_size;

        dir.children.values().for_each(|index| {
            size = self.size(size, index);
        });

        size
    }
}

#[derive(Debug)]
enum Command {
    Cd { target: CdTarget },
    Ls { output: Vec<DirectoryListing> },
}

impl FromStr for Command {
    type Err = ValueError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ValueError(s.into()));
        }
        match &s[1..=2] {
            "cd" => Ok(Command::Cd {
                target: match s[4..].trim() {
                    "/" => CdTarget::Root,
                    ".." => CdTarget::Parent,
                    name => CdTarget::Child(name.into()),
                },
            }),
            "ls" => {
                let files = s
                    .lines()
                    .skip(1)
                    .map(|line| {
                        if line.starts_with("dir") {
                            DirectoryListing::Directory {
                                name: line[4..].trim().to_string(),
                            }
                        } else {
                            let mut parts = line.split_whitespace();
                            DirectoryListing::File {
                                size: parts
                                    .next()
                                    .expect("file size")
                                    .parse()
                                    .expect("valid file size"),
                                name: parts.next().expect("file name").trim().into(),
                            }
                        }
                    })
                    .collect();
                Ok(Command::Ls { output: files })
            }
            _ => Err(ValueError(s.to_string())),
        }
    }
}

#[derive(Debug)]
enum CdTarget {
    Root,
    Parent,
    Child(String),
}

#[derive(Debug)]
enum DirectoryListing {
    File { size: u32, name: String },
    Directory { name: String },
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day7.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> Box<dyn Iterator<Item = Command> + '_> {
    Box::new(input.split('$').filter_map(|s| s.parse().ok()))
}

fn build_file_system(input: Box<dyn Iterator<Item = Command> + '_>) -> FileSystem {
    let mut directories = Arena::<Directory>::new();
    let root = directories.add(Directory::new(None));

    let mut files = Arena::<File>::new();

    input.fold(root, |current_dir, command| match command {
        Command::Cd { target } => match target {
            CdTarget::Root => current_dir,
            CdTarget::Parent => {
                let dir = directories
                    .get(&current_dir)
                    .expect("current directory exists");
                dir.parent.clone().expect("parent directory exists")
            }
            CdTarget::Child(name) => directories
                .get(&current_dir)
                .expect("current directory exists")
                .get_child(name.as_str())
                .expect("parent directory exists"),
        },
        Command::Ls { output } => {
            for listing in output {
                match listing {
                    DirectoryListing::File { size, name } => directories
                        .get_mut(&current_dir)
                        .expect("current directory exists")
                        .add_new_file(&mut files, size, name),
                    DirectoryListing::Directory { name } => {
                        Directory::add_new_directory(&current_dir, &mut directories, name)
                    }
                }
            }
            current_dir
        }
    });

    FileSystem::new(directories, files)
}

fn part1(input: &str) -> u32 {
    let commands = parse_input(input);
    let file_system = build_file_system(commands);
    file_system
        .directories
        .iter()
        .map(|index| file_system.size(0, &index))
        .filter(|&size| size <= 100000)
        .sum()
}

fn part2(input: &str) -> u32 {
    let commands = parse_input(input);
    let file_system = build_file_system(commands);

    let total_space: i32 = 70000000;
    let required_space: i32 = 30000000;
    let used_space: i32 = file_system.size(0, &file_system.root) as i32;
    let available_space: i32 = total_space - used_space;
    let gap: i32 = required_space - available_space;
    assert!(gap > 0);

    file_system
        .directories
        .iter()
        .map(|index| file_system.size(0, &index))
        .filter(|&size| size >= gap as u32)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"$ cd /
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
    fn test_part1() {
        assert_eq!(part1(INPUT), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 24933642);
    }
}
