extern crate cbor;

use std::fs::File;
use std::io::{Write, Read, Seek, SeekFrom};
use std::path::Path;

use cbor::{Decoder, Encoder};

fn main() {
    let path = Path::new("/Users/yuta/IdeaProjects/rust-study/cbor/src/cbor.txt");
    let data = vec![('a', 1), ('b', 2), ('c', 3), ('d', 4), ('e', 5)];

    let mut e = Encoder::from_memory();
    e.encode(&data).unwrap();
    println!("CBOR::Encoder {:?}", e.as_bytes());
    let mut wfile = File::create(&path).unwrap();
    wfile.write_all(e.as_bytes()).unwrap();

    let wbuf: [u8; 4] = [130, 24, 120, 20];
    wfile.seek(SeekFrom::Start(4)).unwrap();
    wfile.write(&wbuf).unwrap();
    println!("CBOR::Encoder {:?}", wbuf);

    let mut rfile = File::open(&path).unwrap();
    rfile.seek(SeekFrom::Start(4)).unwrap();

    let mut buffer: [u8; 4] = [0; 4];
    rfile.read(&mut buffer);
    println!("The bytes: {:?}", buffer);

    //let slice: &[u8] = &buffer;
    let mut d = Decoder::from_bytes(&buffer[0..4]);
    let items: Vec<(char, i32)> = d.decode().collect::<Result<_, _>>().unwrap();
    println!("CBOR::Decoder {:?}", items);
}
