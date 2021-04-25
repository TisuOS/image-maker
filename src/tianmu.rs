#![allow(dead_code)]
#![allow(unused_variables)]
use std::{fs::File, io::{Seek, SeekFrom, Write}, mem::size_of, ptr::slice_from_raw_parts};
use tianmu_fs::{Setting, SuperBlock};
use crate::require::MakeSystem;

pub struct TianMu{
    block_size : usize,
    image_size : usize,
}

impl TianMu {
    pub fn new(block_size : usize, image_size : usize)->Self {
        Self{
            block_size,
            image_size,
        }
    }

    fn find_dir(&mut self, path : &String, image:&mut File)->Result<usize, ()> {
        Err(())
    }
}

impl MakeSystem for TianMu {
    fn add_directory(&self, path : &String, image : &mut std::fs::File) {
    }

    fn add_file(&self, path : &String, image : &mut std::fs::File) {
    }

    fn make_super_block(&self, image : &mut File) {
        let block_num = self.image_size / self.block_size;
        let offset = block_num * 8 + 512;
        let sp = SuperBlock{
            jump1: 0,
            jump2: 0,
            jump3: 0,
            oem: [0;8],
            setting: Setting{
                bytes_per_block: self.block_size as u64,
                block_num: block_num as u64,
                block_map_offset: 512,
                root_table_offset: offset as u64,
            },
        };
        let pos = image.seek(SeekFrom::Start(0)).unwrap();
        let data = &[sp];
        let data = data as *const [SuperBlock] as *const u8;
        let data = slice_from_raw_parts(data, size_of::<SuperBlock>());
        let data = unsafe {&*data};
        let t = image.write(data);
    }
}