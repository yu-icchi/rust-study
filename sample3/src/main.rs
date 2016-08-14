use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let msg: Message = Message::Move {x: 5, y: 9};
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move {x: x, y: y} => move_cursor(x, y),
        Message::Write(s) => println!("{}", s),
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let origin = Point {x: 0, y: 0};
    match origin {
        Point {x, ..} => println!("x is {}", x),
    }

    let c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.area());
    let d = c.grow(2.0).area();
    println!("{}", d);

    let cc = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();
    println!("area: {}", cc.area());
    println!("x: {}", cc.x);
    println!("y: {}", cc.y);

    let v = vec![10, 20, 30, 40, 50];
    for i in v {
        println!("{}", i);
    }

    let mut s = "Hello".to_string();
    println!("{}", s);
    s.push_str(", world.");
    println!("{}", s);
    let ss = s + "!!!";
    println!("{}", ss);
    let hello = "Hello ".to_string();
    let world = "world!".to_string();
    let hello_world = hello + &world;
    println!("{}", hello_world);

    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("five");
            break;
        }
    }

    let pair = (5, -2);
    println!("{:?}", pair);
    match pair {
        (0, y) => println!("{:?}", y),
        (x, 0) => println!("{:?}", x),
        _ => println!("not"),
    }

    // HashMap
    let mut map = HashMap::new();
    map.insert("key01", "value01");
    map.insert("key02", "value02");
    map.insert("key02", "value002");
    map.insert("key03", "value03");
    for (key, &value) in map.iter() {
        println!("map {}: {}", key, value);
    }
    println!("--- remove --");
    map.remove(&"key01");
    for (key, &value) in map.iter() {
        println!("map {}: {}", key, value);
    }
    println!("-- get --");
    match map.get(&"key03") {
        Some(&value) => println!("{}", value),
        None => println!("non"),
    }


}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move {x: i32, y: i32},
    Write(String),
}
fn quit() {
    println!("quit");
}
fn change_color(r:i32, g:i32, b:i32) {
    println!("change_color r:{}, g:{}, b:{}", r, g, b);
}
fn move_cursor(x:i32, y:i32) {
    println!("move_cursor x:{}, y:{}", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius
        }
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn grow(&self, increment: f64) -> Circle {
        Circle {x: self.x, y: self.y, radius: self.radius + increment}
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }
    fn finalize(&self) -> Circle {
        Circle {x: self.x, y: self.y, radius: self.radius}
    }
}