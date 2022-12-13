use std::{io::BufRead, rc::Rc, cell::RefCell, env::current_exe, thread::current, ops::Deref};

#[derive(PartialEq)]
pub enum NodeType {
    File,
    Directory,
}

pub struct TreeNode {
    node_type: NodeType,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
    name: String,
    size: usize,
}

pub fn update_sizes(node: Rc<RefCell<TreeNode>>) -> usize {
    let mut rm = node.borrow_mut();
    println!("updating {0}", rm.name);
    match rm.node_type {
        NodeType::File => {
            println!("size of {0} is {1}", rm.name, rm.size);
            return rm.size;
        },
        NodeType::Directory => {
            println!("{0} is a directory", rm.name);
            let mut sz: usize = 0;
            for n in &rm.children {
                sz += update_sizes(n.clone());
            }
            println!("size of {0} is {sz}", rm.name);
            rm.size = sz;
            return sz;
        },
    }
}

pub fn dfs(node: Rc<RefCell<TreeNode>>, callback: &mut dyn FnMut(Rc<RefCell<TreeNode>>)) {
    callback(node.clone());

    let rm = node.borrow_mut();
    for c in &rm.children {
        dfs(c.clone(), callback);
    }
}

pub fn sum_dirs_le(fs: Rc<RefCell<TreeNode>>, max_size: usize) -> usize {
    let mut dir_sizes: Vec<usize> = Vec::new();
    let mut callback = |node: Rc<RefCell<TreeNode>>| {
        let rm = node.borrow_mut();
        println!("visiting {0}", rm.name);
        if rm.node_type == NodeType::Directory {
            dir_sizes.push(rm.size);
        }
    };
    dfs(fs, &mut callback);

    let mut total_size = 0;
    for dir_size in dir_sizes {
        if dir_size < max_size {
            println!("{dir_size} is less than {max_size}");
            total_size += dir_size;
        }
    }
    return total_size;
}

fn new_file(file_name: &str, file_size: usize, p: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode {
        node_type: NodeType::File,
        children: Vec::new(),
        parent: p,
        name: file_name.to_string(),
        size: file_size
    }))
}

fn new_dir(dir_name: &str, p: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode {
        node_type: NodeType::Directory,
        children: Vec::new(),
        parent: p,
        name: dir_name.to_string(),
        size: 0
    }))
}

pub fn read_fs_tree(reader: &mut dyn BufRead) -> Rc<RefCell<TreeNode>> {
    let root = new_dir("/", None);

    let mut current_dir = root.clone();
    let mut last_dir = root.clone();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trimmed_line = line.trim();
        println!("input line: {trimmed_line}");

        let tok = trimmed_line.split(" ").collect::<Vec<&str>>();

        if trimmed_line.starts_with("$") {
            if tok[1] == "cd" {
                if tok[2] == "/" {
                    // root node - do nothing; root node is added above
                } else if tok[2] == ".." {
                    // go up to parent dir
                    let current_dir_clone = current_dir.clone();
                    current_dir = current_dir_clone.borrow().parent.as_ref().unwrap().clone();
                    //let parent_dir_option = &current_dir.borrow().parent;
                    //let parent_dir_ref = parent_dir_option.as_ref().unwrap();
                    //current_dir = parent_dir_ref.clone();
                    
                    
                    println!("changing directory to {0}", current_dir.borrow().name);
                } else {
                    // go to specified dir
                    let new_dir_name = tok[2];
                    let mut new_dir_node = current_dir.clone();

                    for node in &current_dir.borrow().children {
                        if new_dir_name == node.borrow().name {
                            new_dir_node = node.clone();
                            break;
                        }
                    }

                    last_dir = current_dir.clone();
                    current_dir = new_dir_node;
                    println!("changing directory to {0}", current_dir.borrow().name);
                }
            }
        } else if trimmed_line.starts_with("dir") {
            // create new dir
            let new_node = new_dir(tok[1],
                Some(current_dir.clone()));
            current_dir.borrow_mut().children.push(new_node);
        } else {
            // create new file
            let new_node = new_file(tok[1],
                tok[0].parse::<usize>().unwrap(),
                Some(current_dir.clone()));
            current_dir.borrow_mut().children.push(new_node);
        }
    }

    return root;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_fs_tree_test1() {
        let input = "$ cd /
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
        let mut buf = input.as_bytes();
        let mut fs = read_fs_tree(&mut buf);
        
        assert_eq!("/", fs.borrow().name);
        assert_eq!(0, fs.borrow().size);
        assert_eq!(4, fs.borrow().children.len());

        assert_eq!("a", fs.borrow().children[0].borrow().name);
        assert_eq!(0, fs.borrow().children[0].borrow().size);

        assert_eq!("b.txt", fs.borrow().children[1].borrow().name);
        assert_eq!(14848514, fs.borrow().children[1].borrow().size);
        
        assert_eq!("c.dat", fs.borrow().children[2].borrow().name);
        assert_eq!(8504156, fs.borrow().children[2].borrow().size);
        
        assert_eq!(48381165, update_sizes(fs.clone()));

        assert_eq!(48381165, fs.borrow().size);
        
        let mut dir_sizes: Vec<usize> = Vec::new();
        let mut callback = |node: Rc<RefCell<TreeNode>>| {
            let rm = node.borrow_mut();
            println!("visiting {0}", rm.name);
            if rm.node_type == NodeType::Directory {
                dir_sizes.push(rm.size);
            }
        };
        dfs(fs.clone(), &mut callback);
        assert_eq!(4, dir_sizes.len());

        let mut dirs_le_1e5 = 0;
        for dir_size in dir_sizes {
            if dir_size < 100000 {
                dirs_le_1e5 += 1;
            }
        }
        assert_eq!(2, dirs_le_1e5);

        let sum = sum_dirs_le(fs.clone(), 100000);
        assert_eq!(95437, sum);
    }
}