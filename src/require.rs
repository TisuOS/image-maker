use easy_fs::File;

pub trait MakeSystem {
    fn add_directory(&self, root_path:&String, path : &String, image : &mut File);
    fn add_file(&self, root_path:&String, path : &String, image : &mut File);
    fn make_super_block(&self, image : &mut File);
    fn init_block_map(&self, image : &mut File);
}