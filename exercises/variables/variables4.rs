// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut x = 999;
    println!("Number {}", x);

    // 因为需要 x 可变，需要绑定到 mutable 的变量上
    x = 5; // don't change this line
    println!("Number {}", x);
}
