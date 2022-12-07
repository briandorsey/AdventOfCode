use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use std::collections::HashMap;
use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Dir {
    name: String,
    dirs: HashMap<String, Dir>,
    files: Vec<File>,
}

impl Dir {
    fn size(&self) -> u32 {
        let mut size = 0;
        for file in &self.files {
            size += file.size;
        }
        for dir in self.dirs.values() {
            size += dir.size()
        }
        size
    }
}

impl Display for Dir {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "- {} (dir, size={})", self.name, self.size())?;
        // dirs

        for file in &self.files {
            writeln!(fmt, "  - {} (file, size={})", file.name, file.size)?;
        }

        // TODO: figure out how to print indentation correctly
        for dir in self.dirs.values() {
            write!(fmt, "  {}", dir)?;
        }

        //write!(fmt, "{}",)?;
        Ok(())
    }
}

impl Iterator for Dir {
    type Item = Dir;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct File {
    name: String,
    size: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkingPath {
    root: Dir,
    path: Vec<String>,
}

impl WorkingPath {
    fn borrow_dir(&mut self) -> &mut Dir {
        let mut dir = &mut self.root;
        for e in &self.path {
            //println!("borrow_dir(): walking path: {e}");
            dir = dir.dirs.get_mut(e).expect("borrow_dir");
        }
        dir
    }
}

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let root = Dir {
        name: "/".to_string(),
        dirs: HashMap::new(),
        files: Vec::new(),
    };

    let mut cwd = WorkingPath {
        root,
        path: Vec::new(),
    };

    for execution in input.split("$ ") {
        //println!("{execution}");
        match execution.split_once('\n') {
            Some(("cd /", _)) => cwd.path.clear(),
            Some(("ls", listing)) => {
                //println!("^-MATCH: ls");
                parse_ls(cwd.borrow_dir(), listing);
            }

            Some(("cd ..", _)) => {
                //println!("^-MATCH: cd ..");
                cwd.path.pop();
            }
            Some((cmd, _)) if cmd.starts_with("cd ") => {
                let (_, name) = cmd.split_once(' ').unwrap();

                cwd.path.push(name.to_string());

                //println!("^-MATCH: cd --> {name}")
            }
            Some((l, r)) => println!("^-REMAIN: |{l}|  |{r}|"),
            _ => (),
        }
    }

    //println!("{}", cwd.root);

    let dirs = &mut Vec::new();
    let binding = cwd.root.clone();
    walk_dirs(dirs, &binding);

    dirs.retain(|e| e.size() < 100000);
    println!("part1: {}", dirs.iter().map(|e| e.size()).sum::<u32>());

    let total = 70000000;
    let unused = total - cwd.root.size();
    println!("unused: {unused}");
    let need = 30000000;
    let threshold = need - unused;
    let part2 = &mut Vec::new();
    walk_dirs(part2, &cwd.root);
    let mut part2 = part2.iter().map(|e| e.size()).collect::<Vec<u32>>();
    part2.retain(|e| e >= &threshold);
    println!("part2: {:?}", part2.iter().min().unwrap());
    Ok(())
}

fn walk_dirs<'a>(accum: &mut Vec<&'a Dir>, dir: &'a Dir) {
    //println!("{:?}, {}", dir.name, dir.size());
    accum.push(dir);
    for dir in dir.dirs.values() {
        //println!("{:?}", dir.name);
        walk_dirs(accum, dir);
    }
}

fn parse_ls(dir: &mut Dir, listing: &str) {
    for line in listing.lines() {
        match line.split_once(' ') {
            Some(("dir", name)) => {
                dir.dirs.insert(
                    name.to_string(),
                    Dir {
                        name: name.to_string(),
                        dirs: HashMap::new(),
                        files: Vec::new(),
                    },
                );
            }
            Some((size, name)) => dir.files.push(File {
                size: size.parse::<u32>().unwrap(),
                name: name.to_string(),
            }),
            _ => (),
        }
    }
}
