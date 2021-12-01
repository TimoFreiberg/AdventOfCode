use std::{collections::BTreeMap, iter};

use tracing::debug;

use crate::input;

const TOTAL_FS_SIZE: usize = 70000000;
const FREE_SPACE_REQUIRED: usize = 30000000;

pub fn solve() -> (usize, usize) {
    let input = input(7);
    let fs = parse(&input);
    (part1(&fs), part2(&fs))
}

fn part1(fs: &Fs) -> usize {
    fs.dirs()
        .iter()
        .map(|d| {
            debug!(?d);
            d.size()
        })
        .filter(|dir_size| *dir_size <= 100000)
        .sum()
}

fn part2(fs: &Fs) -> usize {
    let free_space = TOTAL_FS_SIZE - fs.root.size();
    let required_space = FREE_SPACE_REQUIRED - free_space;
    fs.dirs()
        .iter()
        .map(|d| d.size())
        .filter(|dir_size| *dir_size >= required_space)
        .min()
        .unwrap()
}

fn parse(input: &str) -> Fs {
    let mut fs = Fs::new();
    for line in input.lines() {
        let mut words = line.split_ascii_whitespace();
        let mut next = || words.next().unwrap();
        match next() {
            "$" => match next() {
                "cd" => fs.cd(next().to_string()),
                "ls" => continue,
                other => panic!("Command {other:?} not supported"),
            },
            "dir" => fs.add_dir(next().to_string()),
            num => {
                let num = num
                    .parse::<usize>()
                    .unwrap_or_else(|e| panic!("Couldn't parse file size in {line:?}: {e}"));
                fs.add_file(num, next().to_string());
            }
        }
    }
    fs
}

struct Fs {
    root: Dir,
    cwd: Path,
}
impl Fs {
    fn new() -> Self {
        Self {
            root: Default::default(),
            cwd: Default::default(),
        }
    }
    fn cwd(&mut self) -> &mut Dir {
        let mut dir = &mut self.root;
        for p in &self.cwd {
            dir = match dir
                .0
                .get_mut(p)
                .unwrap_or_else(|| panic!("No dir at {p:?} (path: {:?}", self.cwd))
            {
                Node::File { .. } => {
                    panic!("Expected dir, found file at {p:?} (path: {:?})", self.cwd)
                }
                Node::Dir(d) => d,
            };
        }
        dir
    }
    fn cd(&mut self, to: String) {
        match to.as_str() {
            "/" => {
                self.cwd = Vec::new();
            }
            ".." => {
                self.cwd.pop();
            }
            _ => {
                self.add_dir(to.clone());
                self.cwd.push(to.clone());
            }
        }
        debug!(?to, cwd = ?self.cwd, "cd");
    }
    fn add_dir(&mut self, new_dir: String) {
        debug!(cwd=?self.cwd, ?new_dir, "add_dir");
        let dir = self.cwd();
        dir.0.insert(new_dir, Node::Dir(Dir::default()));
    }

    fn add_file(&mut self, size: usize, file_name: String) {
        debug!(cwd=?self.cwd, ?file_name, "add_file");
        let dir = self.cwd();
        dir.0.insert(file_name, Node::File { size });
    }

    fn dirs(&self) -> Vec<&Dir> {
        iter::once(&self.root).chain(self.root.subdirs()).collect()
    }
}

#[derive(Default, Debug)]
struct Dir(BTreeMap<String, Node>);
impl Dir {
    fn size(&self) -> usize {
        self.0
            .values()
            .map(|n| match n {
                Node::File { size } => *size,
                Node::Dir(d) => d.size(),
            })
            .sum()
    }
    fn subdirs(&self) -> Vec<&Dir> {
        self.0
            .values()
            .filter_map(Node::as_dir)
            .flat_map(|d| iter::once(d).chain(d.subdirs()))
            .collect()
    }
}
#[derive(Debug)]
enum Node {
    File { size: usize },
    Dir(Dir),
}
impl Node {
    fn as_dir(&self) -> Option<&Dir> {
        match self {
            Node::File { .. } => None,
            Node::Dir(d) => Some(d),
        }
    }
}
type Path = Vec<String>;

#[test]
fn day7() {
    assert_eq!(solve(), (1086293, 366028))
}
