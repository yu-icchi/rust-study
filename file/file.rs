use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::create("foo.txt").unwrap();
    f.write_all(b"Hello, world!").unwrap();
}