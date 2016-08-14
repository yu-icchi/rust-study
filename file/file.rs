use std::io::prelude::*;
use std::fs::File;

fn main() {
    sample2();
}

fn sample() {
    let mut f = File::create("foo.txt").unwrap();
    f.write_all(b"Hello, world!").unwrap();
}

fn sample2() {
    let mut f = try!(File::create("foo.txt"));
    try!(f.write_all(b"Hello World!!"));
}
