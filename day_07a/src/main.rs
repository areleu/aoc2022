use std::cell::RefCell;
use std::rc::Rc;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Crawler {
    cwd: Rc<RefCell<TreeNode>>,
    memory: Rc<RefCell<TreeNode>>
}

#[derive(Clone)]
pub struct TreeNode {
    pub name: String,
    pub size: Option<i64>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("children", &self.children)
            .field("parent", &self.parent.as_ref().unwrap_or(&Rc::new(RefCell::new(TreeNode{name: String::from("None"), size: None, children: vec![], parent: None}))).borrow().name)
            .finish()
    }
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode {
          name: String::from("/"),
          size: None,
          children: vec![],
          parent: None,
        };
      }

      pub fn calculate_size(&self, treshold: i64) -> i64 {
        let out_size = match self.size {
            Some(v) => {
                        if v <= treshold {
                            v
                        } else {
                            0
                        }
            },
            None => {
                let mut children_size = 0;
                for child in &self.children {
                    children_size += child.borrow().clone().calculate_size(treshold);
                }
                children_size
            }
        };
        out_size
    }

    pub fn add_child(&mut self, child: Rc<RefCell<TreeNode>>) {
        self.children.push(child);
    }
}

pub fn parse_directory(s: &str) -> &str {
    lazy_static! {
        static ref DIRREG: Regex = Regex::new(r"dir (.*)").unwrap();
    }
    let captures = DIRREG.captures(s).unwrap();
    captures.get(1).map_or("", |m| m.as_str())
}

pub fn get_file_info(s: &str) -> (i64, &str) {
    lazy_static! {
        static ref NUMREG: Regex = Regex::new(r"(\d+) (.*)").unwrap();
    }
    let captures = NUMREG.captures(s).unwrap();
    (captures.get(1).map_or(0, |m| m.as_str().parse().unwrap()),
    captures.get(2).map_or("", |m| m.as_str()))
}
impl Crawler {
    pub fn init(root: Rc<RefCell<TreeNode>>, cwd: Option<Rc<RefCell<TreeNode>>>) -> Crawler {
        return Crawler { cwd: match cwd {
                                Some(node) => Rc::clone(&node),
                                None =>  Rc::clone(&root),
                                        },
                         memory: root
    };
    }
    pub fn cd(&mut self, target: &str) {
        if target == ".." {
            let current_name = self.cwd.borrow().name.clone();
            if current_name != "/" {
                let parent_copy = self.cwd.borrow().parent.as_ref().unwrap().clone();
                println!("Moving to: {:?}", parent_copy.borrow().name);
                self.cwd = parent_copy;
                } else {
                    println!("Already in root");
                }
      } else if target == "/" {
                println!("Moving to root");
                let root = self.memory.borrow().clone();
                self.cwd.replace(root);
      } else {
                let dirs: Vec<String> = self.cwd.borrow().children.iter().map(|c| c.borrow().name.clone()).collect();
                let target_str = String::from(target);
                if dirs.contains(&target_str) {
                    let index: usize = {
                                let current_reference = self.cwd.borrow();
                                 current_reference.children.iter().position(|c| c.borrow().name.clone() == target_str).unwrap()
                                };
                    let children_clone = self.cwd.borrow().children[index].clone();
                    println!("Moving to {:?}", target);
                    self.cwd = children_clone;
                } else {
                    let current_name = self.cwd.borrow().name.clone();
                    println!("{:?} not in {:?}", target, current_name);
                }
            }
        }
    pub fn populate(&mut self, ls_output: &Vec<&str>,  folder_stack: &mut Vec<Rc<RefCell<TreeNode>>>) {
        for output in ls_output {
            if String::from(*output).contains("dir") {
                let dir_name = parse_directory(output);
                let current_reference = self.cwd.clone();
                println!("This is the parent of {:?}", dir_name);
                println!("{:?}", &current_reference);
                let new_refcell = Rc::new(RefCell::new(TreeNode{
                    name: String::from(dir_name),
                    size: None,
                    children: vec![],
                    parent: Some(current_reference)
                }));
                let folder_stack_ref = new_refcell.clone();
                println!("This is  {:?}", dir_name);
                println!("{:?}", &new_refcell);
                let push_reference = self.cwd.clone();
                println!("Adding {:?} to {:?}", dir_name, &push_reference.borrow().name);
                push_reference.borrow_mut().children.push(new_refcell);
                folder_stack.push(folder_stack_ref);
            } else {
                let (size, filename) = get_file_info(output);
                let current_reference = self.cwd.clone();
                let new_refcell = Rc::new(RefCell::new(TreeNode{
                    name: String::from(filename),
                    size: Some(size),
                    children: vec![],
                    parent: Some(current_reference)
                }));
                let push_reference = self.cwd.clone();
                println!("Adding {:?} to {:?}", filename, &push_reference.borrow().name);
                push_reference.borrow_mut().children.push(new_refcell);
            }
        }
    }
    }


fn main() {
    let mut lines = include_str!("../example.txt").lines().collect::<Vec<&str>>().into_iter();
    // root directory
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut crawler = Crawler::init(root, None);
    let mut ls_buffer = Vec::<&str>::new();
    let mut ls_flag = false;
    let mut folder_stack = Vec::<Rc<RefCell<TreeNode>>>::new();
    let push_reference = crawler.memory.clone();
    folder_stack.push(push_reference);
    loop {
        let str_form = lines.next().unwrap();
        let current_line: String = String::from(str_form);
        if current_line.contains("$ cd") {
            if ls_flag {
                crawler.populate(&ls_buffer,&mut folder_stack);
                ls_buffer.drain(..);
                ls_flag = false;
            } 
            let target = current_line.split(" ").collect::<Vec<&str>>().pop().unwrap();
            crawler.cd(target);
        } else if current_line.contains("$ ls") {
            ls_flag = true;
        } else {
            ls_buffer.push(str_form)
        }

        if lines.len() == 0 {
            break
        }
    }
    if ls_flag {
        crawler.populate(&ls_buffer,&mut folder_stack);
        ls_buffer.drain(..);
    } 
    let mut size_sum  = 0;
    for folder in &folder_stack {
        let current_size = folder.borrow().clone().calculate_size(10000000000000);
        if current_size <= 100000{
            size_sum += current_size;}
    }
    println!("{:?}",size_sum);
}