struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 整数
    let x:i8 = 64;
    let y = 16;
    println!("x = {}, y = {}", x, y);

    foo(y);
    let yy = add_one(y);
    println!("{}", yy);

    match yy {
        11 => println!("match 11"),
        _ => println!("default")
    }

    // 真偽値
    let b = true;
    println!("b = {}", b);

    // 文字・文字列
    let c = 'x';
    let cc = '❗';
    println!("c = {} {}", c, cc);
    let word = "Hello 世界!!";
    println!("{}", word);

    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr.len());
    let m = &arr[1..3]; // 1, 2
    println!("{}", m.len());

    // 構造体
    let point = Point{x: 1, y: 2};
    println!("{}, {}", point.x, point.y);
}

fn foo(x: i32) {
    println!("{}", x)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

#[test]
fn test_add_one() {
    let x = 9;
    let y = add_one(x);
    assert_eq!(y, 10);
}
