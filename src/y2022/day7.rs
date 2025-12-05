use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::fmt;

pub fn solve(input: &[&str]) -> String {
    let (part1, part2) = file_sizes(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn file_sizes(input: &[&str]) -> (usize, usize) {
    let mut path = vec!["/"];
    let mut size_map: HashMap<String, usize> = HashMap::new();

    for line in input.iter().skip(1) {
        if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("$ cd") {
            let dest = line.split(' ').nth(2).unwrap();
            if dest == ".." {
                path.pop();
            } else {
                path.push(dest);
            }
        } else if line.starts_with("dir") {
            continue;
        } else {
            let (size, _name) = line.split_once(' ').unwrap();
            let size = size.parse::<usize>().unwrap();
            for i in 0..path.len() {
                let key = path
                    .iter()
                    .take(i + 1)
                    .copied()
                    .collect::<Vec<_>>()
                    .join("/")
                    .to_string();
                size_map
                    .entry(key)
                    .and_modify(|s| *s += size)
                    .or_insert(size);
            }
        }
    }

    let mut sizes = size_map
        .iter()
        .map(|(_, size)| *size)
        .collect::<Vec<usize>>();

    let deleatable = sizes.iter().filter(|size| **size <= 100_000).sum();

    let free = 70_000_000 - size_map.get("/").unwrap();
    let need = 30_000_000 - free;

    let mut to_delete = 0;
    sizes.sort();
    for size in sizes {
        if size >= need {
            to_delete = size;
            break;
        }
    }

    (deleatable, to_delete)
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types, dead_code)]
enum ShellCommand {
    cd,
    ls,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum FsItem {
    Dir { name: String },
    File { name: String, size: usize },
}

#[allow(dead_code)]
impl FsItem {
    fn get_name(&self) -> &str {
        match self {
            FsItem::Dir { name } => name,
            FsItem::File { name, size: _ } => name,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            FsItem::Dir { name: _ } => true,
            _ => false,
        }
    }

    fn is_file(&self) -> bool {
        match self {
            FsItem::File { name: _, size: _ } => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
struct FsNode {
    parent: Option<usize>,
    children: Vec<usize>,

    data: FsItem,
}

// #[derive(Debug, PartialEq)]
#[derive(PartialEq)]
struct FileSystem {
    nodes: Vec<FsNode>,
}

impl fmt::Debug for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pretty_format = String::new();
        write!(f, "FileSystem: {{ ")?;
        if f.alternate() {
            pretty_format.push_str("\n   ");
        }
        write!(f, "{pretty_format}nodes: [ ")?;

        for node in &self.nodes {
            if f.alternate() {
                pretty_format.push_str("   ");
            }

            write!(f, "{pretty_format}FsNode {{ ")?;

            if f.alternate() {
                pretty_format.push_str("   ");
            }

            write!(f, "{pretty_format}data: {:?} ", node.data)?;

            if let Some(parent_id) = node.parent {
                match self.get_node_by_id(parent_id) {
                    Ok(parent_node) => {
                        write!(f, "{pretty_format}Parent: Some( parent: {{ ")?;

                        if f.alternate() {
                            pretty_format.push_str("   ");
                        }

                        write!(f, "{pretty_format}id: {parent_id} ")?;
                        write!(f, "{pretty_format}name: {} ", parent_node.data.get_name())?;

                        if f.alternate() {
                            pretty_format.truncate(pretty_format.len() - 3);
                        }

                        write!(f, "{pretty_format}}}), ")?;
                    }
                    Err(err) => {
                        write!(f, "{pretty_format}Some({err}), ")?;
                    }
                }
            } else {
                write!(f, "{pretty_format}Parent: None, ")?;
            }

            write!(f, "{pretty_format}children [ ")?;

            if f.alternate() {
                pretty_format.push_str("   ");
            }

            for child_id in &node.children {
                match self.get_node_by_id(*child_id) {
                    Ok(child_node) => {
                        match child_node.data {
                            FsItem::Dir { name: _ } => write!(f, "{pretty_format}Some( Dir: {{ ")?,
                            FsItem::File { name: _, size } => {
                                write!(f, "{pretty_format}Some( File({size}): {{ ")?
                            }
                        };

                        if f.alternate() {
                            pretty_format.push_str("   ");
                        }

                        write!(f, "{pretty_format}id: {child_id} ")?;
                        write!(f, "{pretty_format}name: {} ", child_node.data.get_name())?;

                        if f.alternate() {
                            pretty_format.truncate(pretty_format.len() - 3);
                        }

                        write!(f, "{pretty_format}}}), ")?;
                    }
                    Err(err) => {
                        write!(f, "{pretty_format}Some({err}), ")?;
                    }
                }
            }

            if f.alternate() {
                pretty_format.truncate(pretty_format.len() - 3);
            }
            write!(f, "{pretty_format}], ")?;

            if f.alternate() {
                pretty_format.truncate(pretty_format.len() - 3);
            }
            write!(f, "{pretty_format}}}, ")?;

            if f.alternate() {
                pretty_format.truncate(pretty_format.len() - 3);
            }
        }

        write!(f, "{pretty_format}], ")?;

        if f.alternate() {
            pretty_format.truncate(pretty_format.len() - 3);
        }
        write!(f, "{pretty_format}}}")
    }
}

#[allow(dead_code)]
impl FileSystem {
    fn new(root: FsItem) -> Self {
        Self {
            nodes: vec![FsNode {
                parent: None,
                children: Vec::new(),

                data: root,
            }],
        }
    }

    fn get_root(&self) -> usize {
        0
    }

    fn add_node(&mut self, data: FsItem, parent_id: usize) -> Result<usize> {
        let index = self.nodes.len();

        let parent_node = self.get_node_by_id_mut(parent_id).context(format!(
            "FileSystem::add_node: could not find parent node `{}`",
            parent_id
        ))?;

        parent_node.children.push(index);

        self.nodes.push(FsNode {
            parent: Some(parent_id),
            children: Vec::new(),
            data,
        });

        Ok(index)
    }

    fn calculate_dir_sizes(&self, start_node: usize) -> anyhow::Result<Vec<usize>> {
        let mut sizes = Vec::new();
        let mut this_size = 0;

        let current_node = self.get_node_by_id(start_node)
            .context("FileSystem::calculate_dir_sizes: Could not find node with id `{}` passed in as starting node")?;

        for child_id in &current_node.children {
            let child_node = self.nodes.get(*child_id)
                .ok_or_else(|| anyhow!("FileSystem::calculate_dir_sizes: could not find node with id `{}` listed is child of node `{}`", child_id, start_node))?;

            if let FsItem::Dir { name: _ } = child_node.data {
                sizes.append(&mut self.calculate_dir_sizes(*child_id)
                         .context(format!("FileSystem::calculate_dir_sizes: Error when calling calculate_dir_sizes from node `{}`", start_node))?);
                if let Some(child_size) = sizes.iter().last() {
                    this_size += child_size
                }
            } else if let FsItem::File { name: _, size } = child_node.data {
                this_size += size;
            }
        }

        sizes.push(this_size);

        Ok(sizes)
    }

    fn get_node_by_id(&self, node_id: usize) -> Result<&FsNode> {
        self.nodes.get(node_id).ok_or_else(|| {
            anyhow!(
                "FileSystem::get_node_by_id: FileSystem does not contain a node with id `{}`",
                node_id
            )
        })
    }

    fn get_node_by_id_mut(&mut self, node_id: usize) -> Result<&mut FsNode> {
        self.nodes.get_mut(node_id).ok_or_else(|| {
            anyhow!(
                "FileSystem::get_node_by_id: FileSystem does not contain a node with id `{}`",
                node_id
            )
        })
    }

    fn get_node_by_name(&self, name: &str, start_node: usize) -> Result<(usize, &FsNode)> {
        let current_node = self.get_node_by_id(start_node)?;
        if current_node.data.get_name() == name {
            return Ok((start_node, current_node));
        }

        for child in &current_node.children {
            if let Ok(node) = self.get_node_by_name(name, *child) {
                return Ok(node);
            }
        }

        Err(anyhow!("FileSystem::get_node_by_name: Could not find a node with name `{name}` starting from node `{:#?}`", self.get_node_by_id(start_node)?))
    }

    fn get_dir_by_name(&self, name: &str, start_node: usize) -> Result<(usize, &FsNode)> {
        let current_node = self.get_node_by_id(start_node)?;
        if current_node.data.get_name() == name {
            return Ok((start_node, current_node));
        }

        for child in &current_node.children {
            if let Ok((node_id, node)) = self.get_node_by_name(name, *child) {
                if node.data.is_dir() {
                    return Ok((node_id, node));
                }
            }
        }

        Err(anyhow!("FileSystem::get_node_by_name: Could not find a node with name `{name}` starting from node `{:#?}`", self.get_node_by_id(start_node)?))
    }
}

#[allow(dead_code)]
fn construct_fs_from_log(log: &[&str]) -> Result<FileSystem> {
    let mut fs = FileSystem::new(FsItem::Dir {
        name: "/".to_string(),
    });

    let root = fs.get_root();
    let mut current_node = root;

    dbg!(&fs);

    for (i, log_line) in log.iter().enumerate() {
        println!("{i}: {log_line}");

        if log_line.starts_with('$') {
            let to_parse = log_line.split(' ').nth(1).ok_or_else(|| {
                anyhow!(
                    "construct_fs_from_log: Could not parse command from line {}: `{log_line}`",
                    i + 1
                )
            })?;

            let command = parse_command(to_parse)?;

            if command == ShellCommand::cd {
                let command_arg = log_line.split(' ').nth(2).ok_or_else(|| {
                    anyhow!("construct_fs_from_log: Could not parse command argument from line {}: `{log_line}`", i+1)
                })?;

                match command_arg {
                    "." => (),
                    ".." => {
                        let parent_name = fs
                            .get_node_by_id(fs.get_node_by_id(current_node)?.parent.unwrap())?
                            .data
                            .get_name();
                        println!("cd {}", parent_name);

                        current_node = fs.get_node_by_id(current_node)?.parent.ok_or_else(|| {
                            anyhow!("construct_fs_from_log: Tried to `cd ..` on a node with no parent. current_node: `{}`", current_node)
                        })?;

                        println!(
                            "{}  pwd: {}, id: {}",
                            i + 1,
                            fs.get_node_by_id(current_node)?.data.get_name(),
                            current_node
                        );
                    }
                    _ => {
                        (current_node, _) =
                            fs.get_dir_by_name(command_arg, current_node)
                                .context(format!(
                                    // "construct_fs_from_log: Error getting node to cd into, line {}: `{log_line}`", i+1
                                    "construct_fs_from_log: Error getting node to cd into, line {}: `{log_line}`\n fs: {:#?}", i+1, fs
                                ))?;

                        let current_node_name = fs.get_node_by_id(current_node)?.data.get_name();
                        println!("cd {}", current_node_name);

                        println!(
                            "{}  pwd: {}, id: {}",
                            i + 1,
                            fs.get_node_by_id(current_node)?.data.get_name(),
                            current_node
                        );
                    }
                }
            }
        } else if log_line.starts_with("dir") {
            let dir_name = log_line.split(' ').nth(1).ok_or_else(|| {
                anyhow!(
                    "construct_fs_from_log: Could not parse dir name from line `{}`",
                    log_line
                )
            })?;

            if fs.get_node_by_name(dir_name, current_node).is_err() {
                fs.add_node(
                    FsItem::Dir {
                        name: dir_name.to_owned(),
                    },
                    current_node,
                )?;
            }
        } else {
            let (file_size, file_name) = log_line.split_once(' ').ok_or_else(|| {
                anyhow!(
                    "Could not parse (file_size, file_name) from line `{}`",
                    log_line
                )
            })?;

            fs.add_node(
                FsItem::File {
                    name: file_name.to_owned(),
                    size: file_size.parse::<usize>()?,
                },
                current_node,
            )?;
        }
    }

    Ok(fs)
}

#[allow(dead_code)]
fn find_deletable_size(log: &[&str]) -> Result<usize> {
    let fs = construct_fs_from_log(log)?;

    let sizes = fs.calculate_dir_sizes(fs.get_root())?;

    Ok(sizes.iter().filter(|size| **size <= 100000).sum::<usize>())
}

#[allow(dead_code)]
fn parse_command(command: &str) -> Result<ShellCommand> {
    match command {
        "cd" => Ok(ShellCommand::cd),
        "ls" => Ok(ShellCommand::ls),
        _ => Err(anyhow!("Could not parse {} as a ShellCommand", command)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_deletable_size() {
        #[rustfmt::skip]
        let log = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];

        let expected = 95437;
        let actual = find_deletable_size(&log).unwrap();

        assert_eq!(actual, expected, "Got `{actual}` when expecting `{expected}` from calling find_deletable_size on `{:#?}`", log)
    }

    #[test]
    fn test_construct_fs_from_log() {
        #[rustfmt::skip]
        let log = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];

        let fs = construct_fs_from_log(&log).unwrap();
        let root = fs.get_root();

        let (a, node) = fs.get_node_by_name("a", root).unwrap();
        assert_eq!(
            node.data,
            FsItem::Dir {
                name: "a".to_string()
            }
        );

        let (_node_id, node) = fs.get_node_by_name("b.txt", root).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "b.txt".to_string(),
                size: 14848514
            }
        );

        let (_node_id, node) = fs.get_node_by_name("c.dat", root).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "c.dat".to_string(),
                size: 8504156
            }
        );

        let (d, node) = fs.get_node_by_name("d", root).unwrap();
        assert_eq!(
            node.data,
            FsItem::Dir {
                name: "d".to_string()
            }
        );

        let (e, node) = fs.get_node_by_name("e", a).unwrap();
        assert_eq!(
            node.data,
            FsItem::Dir {
                name: "e".to_string()
            }
        );

        let (_node_id, node) = fs.get_node_by_name("f", a).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "f".to_string(),
                size: 29116
            }
        );

        let (_node_id, node) = fs.get_node_by_name("g", a).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "g".to_string(),
                size: 2557
            }
        );

        let (_node_id, node) = fs.get_node_by_name("h.lst", a).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "h.lst".to_string(),
                size: 62596
            }
        );

        let (_node_id, node) = fs.get_node_by_name("i", e).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "i".to_string(),
                size: 584
            }
        );

        let (_node_id, node) = fs.get_node_by_name("j", d).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "j".to_string(),
                size: 4060174,
            }
        );

        let (_node_id, node) = fs.get_node_by_name("k", d).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "k".to_string(),
                size: 7214296,
            }
        );

        let (_node_id, node) = fs.get_node_by_name("d.log", d).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "d.log".to_string(),
                size: 8033020,
            }
        );

        let (_node_id, node) = fs.get_node_by_name("d.ext", d).unwrap();
        assert_eq!(
            node.data,
            FsItem::File {
                name: "d.ext".to_string(),
                size: 5626152,
            }
        );

        let actual_size = fs.calculate_dir_sizes(root).unwrap();
        let expected_size = vec![584, 94853, 24933642, 48381165];
        assert_eq!(actual_size, expected_size);
    }

    #[test]
    fn test_file_system_calculate_dir_sizes() {
        let (fs, root, _a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _d_log, _d_ext) =
            construct_test_fs();

        let expected = vec![584, 94853, 24933642, 48381165];
        let actual = fs.calculate_dir_sizes(root).unwrap();

        assert_eq!(
            actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from calling calculate_dir_sizes on `{:#?}` at node `{}`",
            actual, expected, fs, root
        );
    }

    #[test]
    fn test_file_system_get_node_by_name() {
        #[allow(unused_variables)]
        let (fs, root, a, b, c, d, e, f, g, h, i, j, k, d_log, d_ext) = construct_test_fs();

        let name = "a";
        let search_node = root;
        let expected = FsNode {
            parent: Some(root),
            children: vec![e, f, g, h],
            data: FsItem::Dir {
                name: 'a'.to_string(),
            },
        };
        let (node_id, actual) = fs.get_node_by_name(name, search_node).unwrap();

        assert_eq!(node_id, a);
        assert_eq!(
            *actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from calling get_node_by_name on `{:#?}` to find node `{}` from node `{}`",
            *actual, expected, fs, name, search_node
        );

        let name = "d.log";
        let search_node = root;
        let expected = FsNode {
            parent: Some(d),
            children: vec![],
            data: FsItem::File {
                name: "d.log".to_string(),
                size: 8033020,
            },
        };
        let (node_id, actual) = fs.get_node_by_name(name, search_node).unwrap();

        assert_eq!(node_id, d_log);
        assert_eq!(
            *actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from calling get_node_by_name on `{:#?}` to find node `{}` from node `{}`",
            *actual, expected, fs, name, search_node
        );

        let name = "i";
        let search_node = a;
        let expected = FsNode {
            parent: Some(e),
            children: vec![],
            data: FsItem::File {
                name: 'i'.to_string(),
                size: 584,
            },
        };
        let (node_id, actual) = fs.get_node_by_name(name, search_node).unwrap();

        assert_eq!(node_id, i);
        assert_eq!(
            *actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from calling get_node_by_name on `{:#?}` to find node `{}` from node `{}`",
            *actual, expected, fs, name, search_node
        );
    }

    #[test]
    fn test_file_system() {
        let (fs, root, a, b, c, d, e, f, g, h, i, j, k, d_log, d_ext) = construct_test_fs();

        let expected = FileSystem {
            nodes: vec![
                FsNode {
                    parent: None,
                    children: vec![a, b, c, d],
                    data: FsItem::Dir {
                        name: '/'.to_string(),
                    },
                },
                FsNode {
                    parent: Some(root),
                    children: vec![e, f, g, h],
                    data: FsItem::Dir {
                        name: 'a'.to_string(),
                    },
                },
                FsNode {
                    parent: Some(a),
                    children: vec![i],
                    data: FsItem::Dir {
                        name: 'e'.to_string(),
                    },
                },
                FsNode {
                    parent: Some(e),
                    children: vec![],
                    data: FsItem::File {
                        name: 'i'.to_string(),
                        size: 584,
                    },
                },
                FsNode {
                    parent: Some(a),
                    children: vec![],
                    data: FsItem::File {
                        name: 'f'.to_string(),
                        size: 29116,
                    },
                },
                FsNode {
                    parent: Some(a),
                    children: vec![],
                    data: FsItem::File {
                        name: 'g'.to_string(),
                        size: 2557,
                    },
                },
                FsNode {
                    parent: Some(a),
                    children: vec![],
                    data: FsItem::File {
                        name: "h.lst".to_string(),
                        size: 62596,
                    },
                },
                FsNode {
                    parent: Some(root),
                    children: vec![],
                    data: FsItem::File {
                        name: "b.txt".to_string(),
                        size: 14848514,
                    },
                },
                FsNode {
                    parent: Some(root),
                    children: vec![],
                    data: FsItem::File {
                        name: "c.dat".to_string(),
                        size: 8504156,
                    },
                },
                FsNode {
                    parent: Some(root),
                    children: vec![j, d_log, d_ext, k],
                    data: FsItem::Dir {
                        name: 'd'.to_string(),
                    },
                },
                FsNode {
                    parent: Some(d),
                    children: vec![],
                    data: FsItem::File {
                        name: "j".to_string(),
                        size: 4060174,
                    },
                },
                FsNode {
                    parent: Some(d),
                    children: vec![],
                    data: FsItem::File {
                        name: "d.log".to_string(),
                        size: 8033020,
                    },
                },
                FsNode {
                    parent: Some(d),
                    children: vec![],
                    data: FsItem::File {
                        name: "d.ext".to_string(),
                        size: 5626152,
                    },
                },
                FsNode {
                    parent: Some(d),
                    children: vec![],
                    data: FsItem::File {
                        name: "k".to_string(),
                        size: 7214296,
                    },
                },
            ],
        };

        assert_eq!(
            FsNode {
                parent: None,
                children: vec![10, 11, 12, 13],
                data: FsItem::Dir {
                    name: 'd'.to_string(),
                },
            },
            FsNode {
                parent: None,
                children: vec![10, 11, 12, 13],
                data: FsItem::Dir {
                    name: 'd'.to_string(),
                },
            },
        );

        assert_eq!(
            FsNode {
                parent: None,
                children: vec![],
                data: FsItem::File {
                    name: "d.ext".to_string(),
                    size: 5626152,
                },
            },
            FsNode {
                parent: None,
                children: vec![],
                data: FsItem::File {
                    name: "d.ext".to_string(),
                    size: 5626152,
                },
            },
        );

        assert_eq!(
            fs, expected,
            "\nGot `{:#?}` when expecting `{:#?}`",
            fs, expected
        );
    }

    #[allow(clippy::type_complexity)]
    fn construct_test_fs() -> (
        FileSystem,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) {
        let mut fs = FileSystem::new(FsItem::Dir {
            name: '/'.to_string(),
        });

        let root = fs.get_root();

        let a = fs
            .add_node(
                FsItem::Dir {
                    name: 'a'.to_string(),
                },
                root,
            )
            .unwrap();

        let e = fs
            .add_node(
                FsItem::Dir {
                    name: 'e'.to_string(),
                },
                a,
            )
            .unwrap();

        let i = fs
            .add_node(
                FsItem::File {
                    name: 'i'.to_string(),
                    size: 584,
                },
                e,
            )
            .unwrap();

        let f = fs
            .add_node(
                FsItem::File {
                    name: 'f'.to_string(),
                    size: 29116,
                },
                a,
            )
            .unwrap();

        let g = fs
            .add_node(
                FsItem::File {
                    name: 'g'.to_string(),
                    size: 2557,
                },
                a,
            )
            .unwrap();

        let h = fs
            .add_node(
                FsItem::File {
                    name: "h.lst".to_string(),
                    size: 62596,
                },
                a,
            )
            .unwrap();

        let b = fs
            .add_node(
                FsItem::File {
                    name: "b.txt".to_string(),
                    size: 14848514,
                },
                root,
            )
            .unwrap();

        let c = fs
            .add_node(
                FsItem::File {
                    name: "c.dat".to_string(),
                    size: 8504156,
                },
                root,
            )
            .unwrap();

        let d = fs
            .add_node(
                FsItem::Dir {
                    name: 'd'.to_string(),
                },
                root,
            )
            .unwrap();

        let j = fs
            .add_node(
                FsItem::File {
                    name: "j".to_string(),
                    size: 4060174,
                },
                d,
            )
            .unwrap();

        let d_log = fs
            .add_node(
                FsItem::File {
                    name: "d.log".to_string(),
                    size: 8033020,
                },
                d,
            )
            .unwrap();

        let d_ext = fs
            .add_node(
                FsItem::File {
                    name: "d.ext".to_string(),
                    size: 5626152,
                },
                d,
            )
            .unwrap();

        let k = fs
            .add_node(
                FsItem::File {
                    name: "k".to_string(),
                    size: 7214296,
                },
                d,
            )
            .unwrap();

        (fs, root, a, b, c, d, e, f, g, h, i, j, k, d_log, d_ext)
    }

    #[test]
    fn test_fs_nested_diplicate_name() {
        let log = vec![
            "$ cd /", "$ ls", "dir a", "dir b", "dir c", "dir dup", "dir e", "$ cd dup", "$ ls",
            "dir f", "dir g", "dir dup", "1111 dup", "$ cd ..", "$ cd e",
        ];

        construct_fs_from_log(&log).unwrap();
    }

    #[test]
    fn test_fs() {
        #[rustfmt::skip]
        let log = vec![
            "$ cd /",
            "$ ls",
            "dir bfbjzfd",
            "dir mbc",
            "dir psqmv",
            "dir qqpgw",
            "59022 rrqzqwl.frp",
            "dir sscj",
            "dir vpfdwq",
            "dir zzp",

            "$ cd psqmv",
            "$ ls",
            "dir cjzpb",
            "169953 ctb.bmm",
            "151614 gsmdbsp.sjz",
            "245528 hmz.qqt",
            "dir jdvsccc",
            "46723 nfbbqvvs.fdn",
            "dir tbcz",

            "$ cd tbcz",
            "$ ls",
            "dir dwpc",
            "dir gdhpqjd",
            "dir lwmjgnh",
            "dir zlw",
            "dir zps",
            "dir zssntdj",
            "dir zzp",

            "$ cd lwmjgnh",
            "$ ls",
            "dir pnhcpprn",
            "dir zbprw",
            "$ cd pnhcpprn",
                "$ ls",
                "dir tcmp",
                "dir zzp",
                
                "$ cd tcmp",
                    "$ ls",
                    "dir czm",
                    "dir ghnv",
                    "259550 zzp.sgm",

                    "$ cd czm",
                        "$ ls",
                        "244258 zzp",
                    "$ cd ..",

                "$ cd ..",

                "$ cd zzp",

                "$ cd ..", 

            "$ cd ..", // leaving pnhcpprn

            "$ cd zbprw",
        ];

        construct_fs_from_log(&log).unwrap();
    }
}

// #[test]
// fn test_fs() {
//     let log = vec![
//         "$ cd /",
//         "$ ls",
//         "dir bfbjzfd",
//         "dir mbc",
//         "dir psqmv",
//         "dir qqpgw",
//         "59022 rrqzqwl.frp",
//         "dir sscj",
//         "dir vpfdwq",
//         "dir zzp",
//         // "$ cd bfbjzfd",
//         // "$ ls",
//         // "125000 bmzjjgzc.dcr",
//         // "dir brmgzjp",
//         // "165351 hgm",
//         // "dir rhrqttg",
//         // "dir zfdc",
//         // "$ cd brmgzjp",
//         // "$ ls",
//         // "298676 zzp.wrm",
//         // "$ cd ..",
//         // "$ cd rhrqttg",
//         // "$ ls",
//         // "dir hmz",
//         // "dir hpcrbfq",
//         // "$ cd hmz",
//         // "$ ls",
//         // "297949 lqcg",
//         // "$ cd ..",
//         // "$ cd hpcrbfq",
//         // "$ ls",
//         // "dir ghmfgn",
//         // "dir pnhcpprn",
//         // "dir wgvqhw",
//         // "$ cd ghmfgn",
//         // "$ ls",
//         // "dir rrfg",
//         // "dir tggwct",
//         // "$ cd rrfg",
//         // "$ ls",
//         // "240584 svzjf",
//         // "$ cd ..",
//         // "$ cd tggwct",
//         // "$ ls",
//         // "dir lqmrbncv",
//         // "$ cd lqmrbncv",
//         // "$ ls",
//         // "268356 zlw.tsd",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd pnhcpprn",
//         // "$ ls",
//         // "138460 sjth.rhr",
//         // "$ cd ..",
//         // "$ cd wgvqhw",
//         // "$ ls",
//         // "144558 zphv.mwp",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd zfdc",
//         // "$ ls",
//         // "173854 fgj.fhz",
//         // "233205 vwrmp.rzc",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd mbc",
//         // "$ ls",
//         // "93070 jfl",
//         // "95270 jpr.wlb",
//         // "30324 rmrtw",
//         // "243981 zphv.mwp",
//         // "$ cd ..",
//         "$ cd psqmv",
//         "$ ls",
//         "dir cjzpb",
//         "169953 ctb.bmm",
//         "151614 gsmdbsp.sjz",
//         "245528 hmz.qqt",
//         "dir jdvsccc",
//         "46723 nfbbqvvs.fdn",
//         "dir tbcz",
//         // "$ cd cjzpb",
//         // "$ ls",
//         // "156179 lqmrbncv.jpf",
//         // "$ cd ..",
//         // "$ cd jdvsccc",
//         // "$ ls",
//         // "240937 lqmrbncv",
//         // "dir pnhcpprn",
//         // "279509 qpjtdqfg.rwm",
//         // "265236 vwrmp.rzc",
//         // "dir zwjqnf",
//         // "148420 zzp.smp",
//         // "281537 zzp.zhh",
//         // "$ cd pnhcpprn",
//         // "$ ls",
//         // "123983 zphv.mwp",
//         // "$ cd ..",
//         // "$ cd zwjqnf",
//         // "$ ls",
//         // "244132 gsmdbsp.sjz",
//         // "dir stbgzg",
//         // "$ cd stbgzg",
//         // "$ ls",
//         // "9605 cbzw",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd ..",
//         "$ cd tbcz",
//         "$ ls",
//         "dir dwpc",
//         "dir gdhpqjd",
//         "dir lwmjgnh",
//         "dir zlw",
//         "dir zps",
//         "dir zssntdj",
//         "dir zzp",
//         // "$ cd dwpc",
//         // "$ ls",
//         // "225827 jmjtjrnt.zsb",
//         // "30125 zphv.mwp",
//         // "$ cd ..",
//         // "$ cd gdhpqjd",
//         // "$ ls",
//         // "dir lqmrbncv",
//         // "$ cd lqmrbncv",
//         // "$ ls",
//         // "167746 nwhbvps",
//         // "$ cd ..",
//         // "$ cd ..",
//         "$ cd lwmjgnh",
//         "$ ls",
//         "190409 bwdlz.sjf",
//         "dir jbrcnsm",
//         "dir pnhcpprn",
//         "dir zbprw",
//         "51030 zjvgm.tqb",
//         // "$ cd jbrcnsm",
//         // "$ ls",
//         // "dir fhtzf",
//         // "dir zzp",
//         // "$ cd fhtzf",
//         // "$ ls",
//         // "dir hmz",
//         // "$ cd hmz",
//         // "$ ls",
//         // "194260 fbbpbpdm.rrn",
//         // "259497 pcqhzfh.gwv",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd zzp",
//         // "$ ls",
//         // "dir dmqjmgsp",
//         // "dir hmz",
//         // "222531 zzp.zvb",
//         // "$ cd dmqjmgsp",
//         // "$ ls",
//         // "118387 mdgrqss",
//         // "34746 zphv.mwp",
//         // "$ cd ..",
//         // "$ cd hmz",
//         // "$ ls",
//         // "dir ljg",
//         // "dir tpnpp",
//         // "250664 trzfnvc",
//         // "dir vmj",
//         // "$ cd ljg",
//         // "$ ls",
//         // "262125 bnwg.spg",
//         // "dir nrqs",
//         // "$ cd nrqs",
//         // "$ ls",
//         // "259010 gmvnbthn.shd",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd tpnpp",
//         // "$ ls",
//         // "292722 wqljsz.zzj",
//         // "$ cd ..",
//         // "$ cd vmj",
//         // "$ ls",
//         // "94772 thf.jhp",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd ..",
//         // "$ cd ..",
//         "$ cd pnhcpprn",
//             "$ ls",
//             "dir bvdhw",
//             "145794 gsmdbsp.sjz",
//             "dir gtlvqfwv",
//             "dir mzwcp",
//             "30226 nqdjpgpm.zmd",
//             "dir pvmbp",
//             "dir qtc",
//             "dir snwrdp",
//             "dir tcmp",
//             "272593 wjzlh.mbz",
//             "dir zjhcdbg",
//             "dir zzp",
//
//             // "$ cd bvdhw",
//             //     "$ ls",
//             //     "258434 zzp.jbp",
//             // "$ cd ..",
//
//             // "$ cd gtlvqfwv",
//             //     "$ ls",
//             //     "21710 fzg.tvc",
//             //     "25429 vwrmp.rzc",
//             // "$ cd ..",
//
//             // "$ cd mzwcp",
//             //     "$ ls",
//             //     "271460 lqmrbncv",
//             //     "256282 zphv.mwp",
//             // "$ cd ..",
//
//             // "$ cd pvmbp",
//             //     "$ ls",
//             //     "137453 jmbh.csg",
//             //     "9818 ppbgtcr.rrl",
//             //     "dir zlw",
//
//             //     "$ cd zlw",
//             //         "$ ls",
//             //         "232130 pgwdrp.lnd",
//             //         "dir rgvscq",
//             //         "dir zrns",
//
//             //         // "$ cd rgvscq",
//             //         //     "$ ls",
//             //         //     "107168 gsmdbsp.sjz",
//             //         //     "261303 pnhcpprn.lrt",
//             //         // "$ cd ..",
//             //
//             //         "$ cd zrns",
//             //             "$ ls",
//             //             "156386 jcsnz",
//             //             "dir schnzjrv",
//             //             "dir zhfqhlvh",
//             //             "106788 zphv.mwp",
//             //
//             //             "$ cd schnzjrv",
//             //                 "$ ls",
//             //                 "dir zfhmw",
//
//             //                 "$ cd zfhmw",
//             //                     "$ ls",
//             //                     "dir dfd",
//
//             //                     "$ cd dfd",
//             //                         "$ ls",
//             //                         "103262 rvqtmdb",
//             //                     "$ cd ..",
//
//             //                 "$ cd ..",
//
//             //             "$ cd ..",
//
//             //         "$ cd zhfqhlvh",
//             //             "$ ls",
//             //             "267785 pnhcpprn.vzc",
//             //             "297909 zlw",
//             //         "$ cd ..",
//
//             //         "$ cd ..",
//             //     "$ cd ..",
//             // "$ cd ..",
//
//             // "$ cd qtc",
//             //     "$ ls",
//             //     "dir cndr",
//             //     "dir lqmrbncv",
//             //     "201719 rrqzqwl.frp",
//
//             // "$ cd cndr",
//             //     "$ ls",
//             //     "218298 hmz.rqz",
//             //     "104450 mzqtrz.jsb",
//             // "$ cd ..",
//
//             // "$ cd lqmrbncv",
//             //     "$ ls",
//             //     "dir bzlvtw",
//             //     "61437 gsmdbsp.sjz",
//             //     "dir hmz",
//             //     "129583 mqjzq.fvd",
//             //     "232347 rrqzqwl.frp",
//             //     "121479 tjd.hfr",
//             //     "247144 zphv.mwp",
//             //
//             //     "$ cd bzlvtw",
//             //         "$ ls",
//             //         "256488 gscrll.ttf",
//             //         "301325 gsmdbsp.sjz",
//             //     "$ cd ..",
//             //
//             //     "$ cd hmz",
//             //         "$ ls",
//             //         "118154 ggb.gmm",
//             //     "$ cd ..",
//             //
//             // "$ cd ..",
//
//             // "$ cd ..",
//
//             // "$ cd snwrdp",
//             //     "$ ls",
//             //     "dir hmz",
//             //     "dir lqmrbncv",
//             //     "46060 rrqzqwl.frp",
//             //     "245119 vwrmp.rzc",
//             //     "171585 zlw",
//             //     "185512 zzp",
//             //
//             //     "$ cd hmz",
//             //         "$ ls",
//             //         "51192 lmhzqqw",
//             //     "$ cd ..",
//             //
//             //     "$ cd lqmrbncv",
//             //         "$ ls",
//             //         "dir cwz",
//             //
//             //         "$ cd cwz",
//             //             "$ ls",
//             //             "218678 lfsthrnb.vcm",
//             //         "$ cd ..",
//             //
//             //     "$ cd ..",
//             //
//             // "$ cd ..",
//
//             "$ cd tcmp",
//                 "$ ls",
//                 "dir czm",
//                 "dir ghnv",
//                 "259550 zzp.sgm",
//
//                 "$ cd czm",
//                     "$ ls",
//                     "dir jzbn",
//
//                     "$ cd jzbn",
//                         "$ ls",
//                         "dir mzmf",
//
//                         "$ cd mzmf",
//                             // "$ ls",
//                             // "244258 zzp",
//                         "$ cd ..",
//
//                     "$ cd ..",
//
//                 "$ cd ..",
//
//                 // "$ cd ghnv",
//                 //     "$ ls",
//                 //     "42931 rrqzqwl.frp",
//                 //     "168344 zzp",
//                 // "$ cd ..",
//
//             "$ cd ..",
//
//             "$ cd zjhcdbg",
//                 // "$ ls",
//                 // "301695 fwz.mjb",
//                 // "72201 nfhnb",
//                 // "33600 prpwjp",
//             "$ cd ..",
//
//             "$ cd zzp",
//             //     "$ ls",
//             //     "dir bgmrjcr",
//             //     "dir prdswch",
//             //
//             //     "$ cd bgmrjcr",
//             //         "$ ls",
//             //         "278740 gsmdbsp.sjz",
//             //     "$ cd ..",
//             //
//             //     "$ cd prdswch",
//             //         "$ ls",
//             //         "4297 pnhcpprn.bmn",
//             //     "$ cd ..",
//             //
//             "$ cd ..",
//
//         "$ cd ..", // leaving pnhcpprn
//
//         "$ cd zbprw",
//     ];
//
//     let fs = construct_fs_from_log(&log).unwrap();
// }
//
