use std::io::Read;

use tianmu_fs::SuperBlock;

use crate::{maker::Maker, require::MakeSystem};

impl<T:MakeSystem> Maker<T> {
    pub fn print_super_block(&mut self) {
        let data = &mut [0;1024];
        self.file.read(data);
        let sp = unsafe {&*(data as *const [u8] as *const u8 as *const SuperBlock)};
        println!("{:?}", sp);
    }
}