fn hello() {
    println!("Hello, World!");
}

mod foo;

fn main () {
    hello();
    foo::hello();
    foo::bar::hello();
}