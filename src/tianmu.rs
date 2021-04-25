#![allow(dead_code)]
#![allow(unused_variables)]
use std::{cmp::min, io::SeekFrom, mem::size_of, path::Path, ptr::slice_from_raw_parts};
use easy_fs::File;
use tianmu_fs::{DirItem, MAGIC, SuperBlock};
use crate::require::MakeSystem;

pub struct TianMu{
    block_size : usize,
    image_size : usize,
    block_map_addr : usize,
    root_offset : usize,
    block_num : usize,
}

const END : u64 = !0;

impl TianMu {
    pub fn new(block_size : usize, image_size : usize)->Self {
        let block_num = image_size / block_size;
        let root_offset = (block_num*8 + 1024 + block_size - 1) / block_size * block_size;
        println!("block size {}, block num {}, map at {:x}, root at {:x}",
            block_size, block_num, 1024, root_offset);
        Self{
            block_size,
            image_size,
            block_map_addr : 1024,
            root_offset,
            block_num,
        }
    }

    fn store_file(&self, path : &String, image:&mut File)->usize {
        let p = Path::new(path);
        let ctx = std::fs::read(path).unwrap();
        let data = ctx.as_slice();
        let size = data.len();
        let num = (size + self.block_size - 1) / self.block_size;
        let blocks = self.find_free_block_idx(num, image);
        let mut st = 0;
        for idx in blocks.iter() {
            let ed = min(st + self.block_size, data.len());
            let buf = &data[st..ed];
            let addr = idx * self.block_size;
            image.seek(SeekFrom::Start(addr as u64));
            image.write(buf);
            st += self.block_size;
        }
        self.fill_block_map(&blocks, image);
        *blocks.first().unwrap()
    }

    fn find_dir_addr(&self, path : &String, image:&mut File)->Result<usize, ()> {
        let mut p : Vec<&str> = path.split("/").collect();
        p.remove(p.len() - 1);
        let mut idx = self.root_offset / self.block_size;
        println!("find dir addr {:?}", p);
        for name in p {
            if name.len() == 0 {
                continue;
            }
            let mut name = name.to_string();
            if name.len() > 15 {
                name = name.split_at(15).0.to_string();
            }
            
            let item = self.get_dir_item(
                idx, &name.to_string(), image);
            if item.is_none() {
                println!("find dir addr no {} {}", path, name);
            }
            let item = item.unwrap();
            idx = item.start_block as usize;
        }
        Ok(idx * self.block_size)
    }

    fn add_dir_item(&self, dir_idx : usize, item : DirItem, image:&mut File)->Result<(), ()> {
        let num = self.block_size / size_of::<DirItem>();
        for idx in self.get_block_chain(dir_idx, image).iter() {
            let addr = idx * self.block_size;
            let data : &mut [u8;size_of::<DirItem>()] = &mut [0;size_of::<DirItem>()];
            image.seek(SeekFrom::Start(addr as u64));
            for i in 0..num {
                let len = image.read(data);
                let t = slice_to_val::<DirItem>(data);
                if t.empty() {
                    val_to_slice(data, item);
                    println!("{} add at {:x}",
                        slice_to_string(&item.name), image.position() - len);
                    image.seek(SeekFrom::Current(-(len as i64)));
                    image.write(data);
                    return Ok(());
                }
            }
        }
        let idx = self.expend(dir_idx, image).unwrap();
        let addr = idx * self.block_size;
        let data : &mut [u8;size_of::<DirItem>()] = &mut [0;size_of::<DirItem>()];
        for i in 0..num {
            let len = image.read(data);
            let t = slice_to_val::<DirItem>(data);
            if t.empty() {
                val_to_slice(data, item);
                image.seek(SeekFrom::Current(-(len as i64)));
                image.write(data);
                return Ok(());
            }
        }
        Err(())
    }

    fn expend(&self, idx : usize, image:&mut File)->Result<usize, ()> {
        let mut addr = self.block_map_addr + idx * 8;
        let buf : &mut [u8;8] = &mut [0;8];
        image.seek(SeekFrom::Start(addr as u64));
        image.read(buf);
        let mut flag = slice_to_val::<usize>(buf);
        assert_ne!(flag, 0);
        while flag != END as usize {
            addr = flag * 8 + self.block_map_addr;
            image.seek(SeekFrom::Start(addr as u64));
            image.read(buf);
            flag = slice_to_val::<usize>(buf);
        }
        let next = *self.find_free_block_idx(1, image).get(0).unwrap();
        val_to_slice(buf, next);
        image.seek(SeekFrom::Start(addr as u64));
        image.write(buf);
        addr = next * 8 + self.block_map_addr;
        val_to_slice(buf, END);
        image.seek(SeekFrom::Start(addr as u64));
        image.write(buf);
        Ok(next)
    }

    fn fill_block_map(&self, blocks: &Vec<usize>, image:&mut File) {
        let mut next = END;
        for idx in blocks.iter().rev() {
            let addr = self.block_map_addr + idx * 8;
            image.seek(SeekFrom::Start(addr as u64));
            let buf = &mut [0;8];
            val_to_slice(buf, next);
            image.write(buf);
            next = *idx as u64;
        }
    }

    fn find_free_block_idx(&self, num:usize, image:&mut File)->Vec<usize> {
        let st = self.block_map_addr;
        image.seek(SeekFrom::Start(st as u64));
        let mut rt = Vec::new();
        let buf : &mut [u8;8] = &mut [0;8];
        for i in 0..self.block_num {
            let tt = image.read(buf);
            let flag = slice_to_val::<u64>(buf);
            if flag == 0 {
                rt.push(i);
            }
            if rt.len() == num {
                return rt;
            }
        }
        assert_eq!(rt.len(), num);
        rt
    }

    fn get_dir_item(&self, dir_idx : usize, name:&String, image:&mut File)->Option<DirItem> {
        let num = self.block_size / size_of::<DirItem>();
        for idx in self.get_block_chain(dir_idx, image).iter() {
            let addr = idx * self.block_size;
            let data : &mut [u8;size_of::<DirItem>()] = &mut [0;size_of::<DirItem>()];
            image.seek(SeekFrom::Start(addr as u64));
            for i in 0..num {
                image.read(data);
                let item = slice_to_val::<DirItem>(data);
                let iname = slice_to_string(&item.name);
                if &iname == name {
                    return Some(item);
                }
            }
        }
        None
    }

    fn get_block_chain(&self, st_block : usize, image:&mut File)->Vec<usize> {
        let mut block_chain = Vec::new();
        let mut addr = self.block_map_addr + st_block * 8;
        block_chain.push(st_block);
        while addr != 0 {
            image.seek(SeekFrom::Start(addr as u64));
            let data = &mut [0;8];
            image.read(data);
            let t = slice_to_val::<usize>(data);
            if t != 0 && t != END as usize {
                block_chain.push(t);
                addr = t * 8;
            }
            else {
                break;
            }
        }
        block_chain
    }
}

fn slice_to_string(s : &[u8])->String {
    let mut v = Vec::new();
    for c in s {
        if *c == 0 {
            break;
        }
        v.push(*c as u16);
    }
    String::from_utf16(v.as_slice()).unwrap()
}

fn slice_to_val<T:Copy>(s : &[u8])->T {
    unsafe {
        let t = s as *const [u8] as *const u8 as *const T;
        *t
    }
}

fn val_to_slice<T>(s : &mut[u8], v : T) {
    unsafe {
        let t = s as *mut [u8] as *mut u8 as *mut T;
        *t = v;
    }
}

    /// 需传入相对路径
impl MakeSystem for TianMu {
    fn add_directory(&self, root_path:&String, path : &String, image : &mut File) {
        let t : Vec<&str> = path.split("/").collect();
        let dirname = t.last().unwrap().to_string();
        let path = path.split_once(root_path).unwrap().1;
        let dir_idx = self.find_dir_addr(
            &path.to_string(), image).unwrap() / self.block_size;
        let blocks = self.find_free_block_idx(1, image);
        self.fill_block_map(&blocks, image);
        let item = DirItem::new_dir(
            &dirname, *blocks.first().unwrap(), 0);
        self.add_dir_item(dir_idx, item, image).unwrap();
    }

    fn add_file(&self, root_path:&String, path : &String, image : &mut File) {
        let t : Vec<&str> = path.split("/").collect();
        let filename = t.last().unwrap().to_string();
        let size = File::open(path, easy_fs::Option::ReadOnly).size();
        let start_block = self.store_file(path, image);
        let path = path.split_once(root_path).unwrap().1;
        let dir_idx = self.find_dir_addr(
            &path.to_string(), image).unwrap() / self.block_size;
        println!("dir idx {}", dir_idx);
        let item = DirItem::new_file(&filename, start_block, size);
        self.add_dir_item(dir_idx, item, image).unwrap();
    }

    fn make_super_block(&self, image : &mut File) {
        let sp = SuperBlock::new(
            self.block_size, self.block_num, self.block_map_addr, self.root_offset
        );
        let pos = image.seek(SeekFrom::Start(0));
        let data = &[sp];
        let data = data as *const [SuperBlock] as *const u8;
        let data = slice_from_raw_parts(data, size_of::<SuperBlock>());
        let data = unsafe {&*data};
        let mut cnt = 0;
        cnt += image.write(data);
        let mut v = Vec::<u8>::new();
        for _ in 0..(512 - cnt) {
            v.push(0);
        }
        cnt += image.write(v.as_slice());
        let magic : &mut [u8;8] = &mut [0;8];
        val_to_slice(magic, MAGIC);
        cnt += image.write(magic);
        for i in 0..((1024 - cnt) - v.len()) {
            v.push(0);
        }
        cnt += image.write(v.as_slice());
        assert_eq!(cnt, 1024)
    }

    fn init_block_map(&self, image : &mut File) {
        let mut t = Vec::<u8>::new();
        for _ in 0..self.block_size {
            t.push(0);
        }
        let buf = t.as_slice();
        image.seek(SeekFrom::Start(0));
        for _ in 0..self.block_num {
            image.write(buf);
        }
        image.seek(SeekFrom::Start(self.block_map_addr as u64));
        let buf = &[0;8];
        let num = self.block_size / 8;
        let num = (self.block_num + num - 1) / num * num;
        for i in 0..num {
            image.write(buf);
        }
        image.seek(SeekFrom::Start(self.block_map_addr as u64));
        let num = self.root_offset / self.block_size + 1;
        let buf : &[u8;8] = &[0xff;8];
        for _ in 0..num {
            image.write(buf);
        }
    }
}