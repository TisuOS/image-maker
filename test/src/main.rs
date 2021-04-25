use std::env;

use crate::{maker::Maker, tianmu::TianMu};

mod maker;
mod require;
mod tianmu;
mod test;

const SIZE : usize = 1024 * 1024 * 8;

fn main() {
    assert!(env::args().len() == 3);
    let args : Vec<String> = env::args().collect();
    let src = args.get(1).unwrap();
    let target = args.get(2).unwrap();
    let format = TianMu::new(4096 * 4, SIZE);
    let mut mk = Maker::new(src.clone(), target.clone(), format);
    mk.make();
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let format = TianMu::new(4096 * 4, SIZE);
        let mut mk = Maker::read(
            String::from("./"), String::from("img"), format);
        mk.print_super_block();
    }
}