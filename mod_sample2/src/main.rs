extern crate mod_sample2;

use mod_sample2::foo;

fn hello() {
    println!("Hello, World!");
}

fn main() {
    hello();
    foo::hello();
    foo::greet();
}
