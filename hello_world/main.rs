/// #main
///
/// 足し算関数を実行させる
///
fn main() {
    let x: i32 = add(5, 4);
    println!("Hello World! {}", x);
}

// 足し算関数
fn add(a: i32, b: i32) -> i32 {
    a + b
}
