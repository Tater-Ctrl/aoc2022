use std::{
    // borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct File {
    pub name: String,
    pub size: u64,
}

pub struct Directory {
    pub name: String,
    pub parent: RefCell<Weak<Directory>>,
    pub subdirectories: RefCell<Vec<Rc<Directory>>>,
    pub files: Rc<RefCell<Vec<File>>>,
    pub size: u64,
}

impl Directory {
    pub fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
            subdirectories: RefCell::new(Vec::new()),
            files: Rc::new(RefCell::new(Vec::new())),
            size: 0,
        }
    }

    pub fn delete(&self) {
        let parent = self.parent.borrow().upgrade();
        match parent {
            Some(parent) => {
                let mut subdirectories = parent.subdirectories.borrow_mut();
                let index = subdirectories
                    .iter()
                    .position(|dir| dir.name == self.name)
                    .unwrap();
                subdirectories.remove(index);
            }

            None => return,
        }
    }

    pub fn dir_below_size(&self, size: u64) -> Vec<u64> {
        let mut sizes: Vec<u64> = Vec::new();
        let recursive_size: u64 = self.dir_size();

        if recursive_size < size {
            sizes.push(recursive_size);
        }

        self.subdirectories
            .borrow()
            .iter()
            .for_each(|dir| sizes.append(&mut dir.dir_below_size(size)));
        sizes
    }

    pub fn dir_greater_than(&self, size: u64, list: &mut Vec<Weak<Directory>>) {
        let dir_size: u64 = self.dir_size();
        let dir = self.parent.borrow().upgrade();

        self.subdirectories
            .borrow()
            .iter()
            .for_each(|dir| dir.dir_greater_than(size, list));

        if dir_size < size {
            match dir {
                Some(dir) => {
                    if dir.dir_size() >= size {
                        list.push(Rc::downgrade(&dir));
                    }
                }
                None => return,
            }
        }
    }

    pub fn dir_size(&self) -> u64 {
        let mut size = 0;

        self.files.borrow().iter().for_each(|file| {
            size += file.size;
        });

        self.subdirectories
            .borrow()
            .iter()
            .for_each(|dir| size += dir.dir_size());

        size
    }
}

pub struct Explorer {
    root: Rc<Directory>,
    pub current_dir: Weak<Directory>,
}

impl Explorer {
    pub fn new(name: &str) -> Explorer {
        let mut explorer = Explorer {
            root: Rc::new(Directory::new(name)),
            current_dir: Weak::new(),
        };

        explorer.current_dir = Rc::downgrade(&explorer.root);
        explorer
    }

    pub fn dir_name(&self) -> String {
        self.current_dir.upgrade().unwrap().name.clone()
    }

    pub fn create_dir(&mut self, name: &str) {
        let current_dir = self.current_dir.upgrade().unwrap();
        let new_dir = Rc::new(Directory::new(name));
        *new_dir.parent.borrow_mut() = Rc::downgrade(&current_dir);
        current_dir.subdirectories.borrow_mut().push(new_dir);
    }

    pub fn create_file(&mut self, name: &str, size: u64) {
        let current_dir = self.current_dir.upgrade().unwrap();
        current_dir.files.borrow_mut().push(File {
            name: name.to_string(),
            size,
        });
    }

    pub fn get_files(&self) -> Rc<RefCell<Vec<File>>> {
        let current_dir = self.current_dir.upgrade().unwrap();
        Rc::clone(&current_dir.files)
    }

    pub fn dir_below_size(&self, size: u64) -> Vec<u64> {
        self.current_dir.upgrade().unwrap().dir_below_size(size)
    }

    pub fn dir_size(&self) -> u64 {
        self.current_dir.upgrade().unwrap().dir_size()
    }

    pub fn dir_greater_than(&self, size: u64, list: &mut Vec<Weak<Directory>>) {
        self.current_dir
            .upgrade()
            .unwrap()
            .dir_greater_than(size, list);
    }

    pub fn move_to_root(&mut self) {
        self.current_dir = Rc::downgrade(&self.root);
    }

    pub fn move_up(&mut self) {
        let parent = self
            .current_dir
            .upgrade()
            .unwrap()
            .parent
            .borrow()
            .upgrade()
            .unwrap();
        self.current_dir = Rc::downgrade(&parent);
    }

    pub fn move_down(&mut self, name: &str) {
        let current_dir = self.current_dir.upgrade().unwrap();
        let binding = current_dir.subdirectories.borrow();
        let child = binding.iter().find(|child| child.name == *name);

        match child {
            Some(child) => self.current_dir = Rc::downgrade(child),
            None => println!("Directory not found"),
        }
    }
}
