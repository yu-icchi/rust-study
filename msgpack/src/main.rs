extern crate rmp as msgpack;

use msgpack::Encoder;

fn main() {
    let val = (42u8, "the Answer");

    // The encoder borrows the bytearray buffer.
    let mut buf = [0u8; 13];

    val.encode(&mut Encoder::new(&mut &mut buf[..]));

    assert_eq!([0x92, 0x2a, 0xaa, 0x74, 0x68, 0x65, 0x20, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72], buf);
}