#![allow(dead_code)]
use std::{fs, path::Path};
use easy_fs::File;
use crate::require::MakeSystem;

pub struct Maker<T: MakeSystem> {
    pub root_path : String,
    pub target_path : String,
    pub format : T,
    pub file : File,
}

impl<T: MakeSystem> Maker<T> {
    pub fn new(root_path : String, target_path : String, format: T)->Self {
        let file = File::open(&target_path, easy_fs::Option::ReadWrite);

        Self {
            root_path,
            target_path,
            format,
            file,
        }
    }

    pub fn read(root_path : String, target_path : String, format: T)->Self {
        let file = File::open(&target_path, easy_fs::Option::ReadOnly);

        Self {
            root_path,
            target_path,
            format,
            file,
        }
    }

    pub fn make(&mut self) {
        self.format.init_block_map(&mut self.file);
        self.format.make_super_block(&mut self.file);
        self.trace(&self.root_path.clone());
    }

    fn trace(&mut self, dir : &String) {
        for file in self.get_files(dir).iter() {
            self.format.add_file(&self.root_path, file, &mut self.file)
        }
        for dir in self.get_dirs(dir).iter() {
            self.format.add_directory(&self.root_path, dir, &mut self.file);
            self.trace(dir);
        }
    }

    fn get_files(&self, dir : &String)->Vec<String> {
        let mut rt = Vec::new();
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap();
            if Path::is_file(path.path().as_path()) {
                rt.push(path.path().to_str().unwrap().to_string());
            }
        }
        rt
    }

    fn get_dirs(&self, dir : &String)->Vec<String> {
        let mut rt = Vec::new();
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap();
            if Path::is_dir(path.path().as_path()) {
                rt.push(path.path().to_str().unwrap().to_string());
            }
        }
        rt
    }

    fn make_super_block(&mut self) {
        // let f = File::open("").unwrap();
    }
}
