/// #main
///
/// 足し算関数を実行させる
///
fn main() {
    let x: i32 = add(5, 4);
    let y: i32 = sub(x, 2);
    if x > 5 {
        println!("{}", x);
    }
    if y < 10 {
        println!("{}", y);
    }
    println!("Hello World! {} {}", x, y);

    let mut xx = 5;
    let mut done = false;
    while !done {
        xx += xx -3;
        println!("{}", xx);
        if xx % 5 == 0 {
            done = true;
        }
    }
    let p = Point::new();
    p.print(9);
    Point::to_string();

    for i in 0..10 {
        println!("{}", i);
    }
}

struct Point {x: i32, y: i32}

impl Point {
    fn new() -> Point {
        Point {x: 0, y: 0}
    }
    fn print(&self, a: i32) -> &Self {
        println!("print {}", a);
        self
    }
    fn to_string() -> () {
        println!("toString");
    }
}

// 足し算
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 引き算
fn sub(a: i32, b: i32) -> i32 {
    a - b
}
