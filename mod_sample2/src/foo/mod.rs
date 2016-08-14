pub fn hello() {
    println!("Hello, World!!");
}

pub use self::bar::hello as greet;

mod bar;