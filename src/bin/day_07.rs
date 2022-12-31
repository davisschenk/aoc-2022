#![feature(test)]
extern crate test;

use std::collections::HashMap;
use std::path::{Component, PathBuf};

#[derive(Debug)]
enum FileSystem {
    Directory(HashMap<String, Box<Self>>),
    File(usize),
}

impl FileSystem {
    fn new() -> Self {
        Self::Directory(HashMap::new())
    }

    fn insert<T: ToString>(&mut self, name: T, value: Self) -> Option<&mut Box<Self>> {
        match self {
            Self::Directory(hm) => {
                hm.insert(name.to_string(), Box::new(value));
            }
            _ => return None,
        }

        if let Self::Directory(hm) = self {
            hm.get_mut(&name.to_string())
        } else {
            None
        }
    }

    fn insert_directory<T: ToString>(&mut self, name: T) -> &mut Box<Self> {
        let new_dir = FileSystem::Directory(HashMap::new());

        self.insert(name, new_dir).unwrap()
    }

    fn insert_file<T: ToString>(&mut self, name: T, size: usize) {
        let new_file = FileSystem::File(size);

        self.insert(name, new_file);
    }

    #[allow(dead_code)]
    fn get<T: Into<PathBuf>>(&self, path: T) -> Option<&Self> {
        let mut cur: &Self = self;
        let path: PathBuf = path.into();

        for component in path.components() {
            if let Component::Normal(os) = component {
                let key = os.to_string_lossy().to_string();
                if let Self::Directory(ref d) = *cur {
                    cur = d.get(&key).unwrap()
                }
            }
        }

        Some(cur)
    }

    fn get_mut<T: Into<PathBuf>>(&mut self, path: T) -> Option<&mut Self> {
        let mut cur: &mut Self = self;
        let path: PathBuf = path.into();

        for component in path.components() {
            if let Component::Normal(os) = component {
                let key = os.to_string_lossy().to_string();
                if let Self::Directory(ref mut d) = *cur {
                    cur = d.get_mut(&key).unwrap()
                }
            }
        }

        Some(cur)
    }

    fn sum(&self) -> usize {
        let mut total = 0;

        match self {
            Self::File(size) => return *size,
            Self::Directory(dir) => {
                for (_, v) in dir.iter() {
                    match **v {
                        Self::File(size) => total += size,
                        Self::Directory(_) => total += v.sum(),
                    }
                }
            }
        }

        total
    }
}

fn read_directory(input: &str) -> FileSystem {
    let mut cur_path = PathBuf::new();
    let mut directory = FileSystem::new();

    for line in input.lines() {
        let cmd: Vec<&str> = line.split_whitespace().collect();

        match cmd[..] {
            ["$", "cd", ".."] => {
                cur_path.pop();
            }
            ["$", "cd", x] => cur_path.push(x),
            ["$", "ls"] => {}
            [size, name] => {
                let dir = directory.get_mut(&cur_path).unwrap();
                match size {
                    "dir" => {
                        dir.insert_directory(name);
                    }
                    _ => dir.insert_file(name, size.parse().unwrap()),
                }
            }
            _ => eprintln!("Ahh couldnt match"),
        }
    }

    directory
}

fn calculate_answer_one(dir: &FileSystem) -> usize {
    let mut m = 0;

    if let FileSystem::Directory(dir) = dir {
        for (_, v) in dir.iter() {
            if let FileSystem::Directory(_) = **v {
                m += calculate_answer_one(v);

                let s = v.sum();
                if s <= 100000 {
                    m += s;
                }
            }
        }
    }

    m
}

fn part_one(input: &str) -> usize {
    let directory = read_directory(input);
    calculate_answer_one(&directory)
}

fn find_directory(start: &FileSystem, size: usize) -> usize {
    let mut directories: Vec<&FileSystem> = Vec::new();

    fn get_directories<'a>(d: &'a FileSystem, dirs: &mut Vec<&'a FileSystem>) {
        if let FileSystem::Directory(dir) = d {
            dirs.push(d);

            for (_, v) in dir.iter() {
                if let FileSystem::Directory(_) = **v {
                    get_directories(v, dirs);
                }
            }
        }
    }

    get_directories(start, &mut directories);

    directories
        .iter()
        .map(|d| d.sum())
        .filter(|s| s >= &size)
        .min()
        .unwrap()
}

fn part_two(input: &str) -> usize {
    const TOTAL_SPACE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;

    let directory = read_directory(input);
    let disk_usage = directory.sum();
    let free_space = TOTAL_SPACE - disk_usage;
    let minimum_deleted = NEEDED_SPACE - free_space;

    find_directory(&directory, minimum_deleted)
}

utils::aoc_problem!(day_07, part_one, 95437, 1648397, part_two, 24933642, 1815525);

#[cfg(test)]
mod day_07_additional {}
