// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // 1. 直接写死法
    // let a = [1, 2, 3, 4, 5];

    // 2. 直接定义法
    // 不可以这样写，大小给多少就要初始化多少
    // let a: [i32; 101] = [1, 2, 3];

    // 可以直接初始化成相同的值
    let a = [0; 101];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
