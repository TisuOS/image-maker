#![allow(dead_code)]
use std::{fs::{self, File}, path::Path};

use crate::require::MakeSystem;

pub struct Maker<T: MakeSystem> {
    pub root_path : String,
    pub target_path : String,
    pub format : T,
    pub file : File,
}

impl<T: MakeSystem> Maker<T> {
    pub fn new(root_path : String, target_path : String, format: T)->Self {
        let mut file;
        if let Ok(f) = File::open(target_path.clone()) {
            file = f;
        }
        else {
            file = File::create(target_path.clone()).unwrap();
        }
        
        Self {
            root_path,
            target_path,
            format,
            file,
        }
    }

    pub fn make(&mut self) {
        self.format.make_super_block(&mut self.file);
        self.trace(&self.root_path.clone());
    }

    fn trace(&mut self, dir : &String) {
        for file in self.get_files(dir).iter() {
            self.format.add_file(file, &mut self.file)
        }
        for dir in self.get_dirs(dir).iter() {
            self.format.add_directory(dir, &mut self.file)
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
