use std::{collections::HashMap, fs};

type RegularFile = u32;
type DirectoryFile = HashMap<String, File>;

#[derive(Debug)]
enum File {
    Regular(RegularFile),
    Directory(DirectoryFile),
}

impl File {
    fn size(&self) -> u32 {
        match self {
            File::Regular(size) => *size,
            File::Directory(dir) => dir_size(&dir),
        }
    }
}

fn dir_size(dir: &DirectoryFile) -> u32 {
    dir.values().map(|f| f.size()).sum()
}

fn add_file(root: &mut DirectoryFile, path: &Vec<&str>, name: &str, file: File) {
    let mut parent = root;
    for &dir in path {
        parent = match parent.get_mut(dir) {
            Some(File::Directory(inner)) => inner,
            _ => panic!(),
        }
    }
    parent.insert(name.to_string(), file);
}

fn compute_part1(dir: &DirectoryFile) -> u32 {
    dir.values()
        .map(|f| match f {
            File::Directory(inner_dir) => {
                let size = dir_size(inner_dir);
                if size <= 100000 {
                    size + compute_part1(inner_dir)
                } else {
                    compute_part1(inner_dir) // (don't count actual folder)
                }
            }
            _ => 0,
        })
        .sum()
}

fn compute_part2(dir: &DirectoryFile, target: u32) -> Option<u32> {
    dir.values()
        .map(|f| match f {
            File::Directory(inner_dir) => {
                let size = dir_size(inner_dir);
                if size < target {
                    None
                } else {
                    compute_part2(inner_dir, target).or(Some(size))
                }
            }
            _ => None,
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .min()
}

fn main() {
    let input = fs::read_to_string("inputs/07.txt").unwrap();

    let mut root = DirectoryFile::new();
    let mut pwd = Vec::new();

    input.trim().lines().for_each(|line| {
        let parts: Vec<_> = line.split(' ').collect();
        match parts[..] {
            ["$", "cd", name] => {
                match name {
                    "/" => {
                        pwd.clear();
                    }
                    ".." => {
                        pwd.pop();
                    }
                    name => {
                        pwd.push(name);
                    }
                };
            }
            ["$", "ls"] => (),
            ["dir", name] => {
                add_file(&mut root, &pwd, name, File::Directory(DirectoryFile::new()));
            }
            [size, name] => {
                add_file(
                    &mut root,
                    &pwd,
                    name,
                    File::Regular(size.parse::<RegularFile>().unwrap()),
                );
            }
            _ => panic!(),
        }
    });
    // println!("{:?}", root);

    let resp1 = compute_part1(&root);
    println!("{}", resp1);

    let used = dir_size(&root);
    let unused = 70000000 - used;
    let resp2 = compute_part2(&root, 30000000 - unused).unwrap();
    println!("{}", resp2);
}
