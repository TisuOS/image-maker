use std::fs::File;

pub trait MakeSystem {
    fn add_directory(&self, path : &String, image : &mut File);
    fn add_file(&self, path : &String, image : &mut File);
    fn make_super_block(&self, image : &mut File);
}