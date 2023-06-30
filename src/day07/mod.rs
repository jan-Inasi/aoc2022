use std::collections::HashMap;

pub fn solve(input: String, is_part_one: bool) {
    let fs = parse_input(&input);
    let result = if is_part_one {
        solve_part_one(fs)
    } else {
        solve_part_two(fs)
    };

    println!("output: {result}");
}

fn solve_part_one(fs: FileSystem) -> i32 {
    let mut dir_sizes = Vec::<u32>::new();
    fs.dfs_dir_sizes(fs.root_id, &mut dir_sizes);
    dir_sizes.iter().filter(|&&x| x <= 100_000).sum::<u32>() as i32
}

fn solve_part_two(fs: FileSystem) -> i32 {
    let mut dir_sizes = Vec::<u32>::new();
    fs.dfs_dir_sizes(fs.root_id, &mut dir_sizes);
    let root_size = match dir_sizes.last() {
        Some(&value) => value,
        None => {
            println!("WARNING empty dir_sizes");
            return -1;
        }
    };

    println!("root size: {root_size}");

    let avaiable_memory = DISK_SPACE - root_size;
    let memory_to_free = NEEDED_MEMORY - avaiable_memory;

    if memory_to_free <= 0 {
        return 0;
    }

    println!("memory to free: {memory_to_free}");

    dir_sizes.sort();

    match dir_sizes.iter().find(|&&x| x >= memory_to_free) {
        Some(&x) => x as i32,
        None => {
            println!("couldn't find a directory big enought");
            return -1;
        }
    }
}

const DISK_SPACE: u32 = 70_000_000;
const NEEDED_MEMORY: u32 = 30_000_000;

fn parse_input(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    for line in input.split('$') {
        if line.len() >= 3 {
            match &line[1..3] {
                "ls" => {
                    parse_ls(line, &mut fs);
                }
                "cd" => {
                    parse_cd(line, &mut fs);
                }
                _ => {
                    println!("WARNING command not recognized");
                }
            }
        }
    }

    fs
}

struct Directory {
    pub subdirs: HashMap<String, usize>,
    pub files: HashMap<String, u32>,
}

impl Directory {
    pub fn new() -> Self {
        Directory {
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn get_subdir_id(&self, name: &str) -> Option<&usize> {
        self.subdirs.get(name)
    }
}

struct FileSystem {
    directories: Vec<Directory>,
    root_id: usize,
    cwd_id: usize,
    absolute_path: Vec<usize>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            directories: vec![(Directory::new())],
            root_id: 0,
            cwd_id: 0,
            absolute_path: Vec::new(),
        }
    }

    pub fn cd_root(&mut self) {
        self.cwd_id = self.root_id;
        self.absolute_path.clear();
    }

    pub fn parent_dir(&self) -> usize {
        match self.absolute_path.last() {
            Some(id) => *id,
            None => self.root_id,
        }
    }

    pub fn cd_parent(&mut self) {
        self.cwd_id = self.parent_dir();
        self.absolute_path.pop();
    }

    pub fn cwd(&self) -> &Directory {
        &self.directories[self.cwd_id]
    }

    pub fn cwd_mut(&mut self) -> &mut Directory {
        &mut self.directories[self.cwd_id]
    }

    pub fn cd(&mut self, dir_name: &str) -> Result<(), &str> {
        match self.cwd().get_subdir_id(dir_name) {
            None => Err("directory does not exist"),
            Some(&dir_id) => {
                self.absolute_path.push(self.cwd_id);
                self.cwd_id = dir_id;
                Ok(())
            }
        }
    }

    fn next_dir_id(&self) -> usize {
        self.directories.len()
    }

    pub fn mkdir(&mut self, dir_name: &str) -> Result<(), &str> {
        if let Some(_) = self.cwd().get_subdir_id(dir_name) {
            return Err("directory already exists");
        }

        let dir_id = self.next_dir_id();
        self.cwd_mut().subdirs.insert(dir_name.into(), dir_id);
        self.directories.push(Directory::new());

        Ok(())
    }

    pub fn mkfile(&mut self, file_name: &str, file_size: u32) -> Result<(), &str> {
        if let Some(_) = self.cwd().files.get(file_name) {
            return Err("file already exists");
        }
        self.cwd_mut().files.insert(file_name.into(), file_size);
        Ok(())
    }

    pub fn dir(&self, dir_id: usize) -> &Directory {
        &self.directories[dir_id]
    }

    pub fn dfs_dir_sizes(&self, dir_id: usize, dir_sizes: &mut Vec<u32>) -> u32 {
        let dir = self.dir(dir_id);
        let mut size: u32 = dir.files.values().sum();
        for subdir_id in dir.subdirs.values() {
            size += self.dfs_dir_sizes(*subdir_id, dir_sizes)
        }
        dir_sizes.push(size);
        size
    }
}

fn parse_ls(text: &str, fs: &mut FileSystem) {
    for line in text.lines().skip(1) {
        let x: Vec<&str> = line.split(" ").take(2).collect();
        if x.len() < 2 {
            println!("WARNING couldn't parse ls line");
            return;
        }

        match x[0] {
            "dir" => {
                if let Err(err) = fs.mkdir(x[1]) {
                    println!("WARNING {err}");
                }
            }
            file_size_str => match file_size_str.parse() {
                Ok(file_size) => {
                    if let Err(err) = fs.mkfile(x[1], file_size) {
                        println!("WARNING {err}");
                    }
                }
                Err(err) => {
                    println!("WARNING {err}");
                }
            },
        }
    }
}

fn parse_cd(text: &str, fs: &mut FileSystem) {
    let cd_argument = &text[4..text.len() - 1]; // strip new line
    match cd_argument {
        "/" => fs.cd_root(),
        ".." => fs.cd_parent(),
        dir_name => {
            if let Err(err) = fs.cd(dir_name) {
                println!("WARNING {err}");
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let fs = parse_input(INPUT);
        assert_eq!(95437, solve_part_one(fs));
    }
    #[test]
    fn test_example_part_two() {
        let fs = parse_input(INPUT);
        assert_eq!(24933642, solve_part_two(fs));
    }

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
}
