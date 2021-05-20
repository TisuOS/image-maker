use clap::{App, Arg};

use crate::{maker::Maker, tianmu::TianMu};

mod maker;
mod require;
mod tianmu;
mod test;

#[allow(dead_code)]
const SIZE : usize = 1024 * 1024 * 32;

fn main() {
    let matches = App::new("Image maker")
        .version("0.1.3")
        .author("大树之下 <belowthetree>")
        .about("Make an image from a directory with custom format")
        .arg(Arg::with_name("size")
            .short("S")
            .help("Set the image size(mb)")
            .default_value("32")
            .takes_value(true)
        )
        .arg(Arg::with_name("source")
            .short("s")
            .help("Set the source directory")
            .takes_value(true)
        )
        .arg(Arg::with_name("target")
            .short("t")
            .help("path that image will be placed")
            .default_value("./")
            .takes_value(true)
        ).get_matches();
    let size = matches.value_of("size").unwrap();
    let size = from_str(size);
    let src = matches.value_of("source").unwrap().to_string();
    let target = matches.value_of("target").unwrap().to_string();
    let format = TianMu::new(4096 * 4, 1024 * 1024 * size);
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

fn from_str(s : &str)->usize {
    let mut rt = 0;
    for c in s.as_bytes() {
        rt *= 10;
        if *c < '0' as u8 || *c > '9' as u8 {
            panic!("size with wrong char {}", s);
        }
        rt += *c as usize - '0' as usize;
    }
    rt
}