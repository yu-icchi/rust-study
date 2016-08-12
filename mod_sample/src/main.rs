fn hello() {
    println!("Hello, world!");
}

mod foo;

fn main() {
    hello();
    foo::hello();
    foo::bar::hello();
}
