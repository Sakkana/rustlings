// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // 所有权在 data
    let data = "Rust is great!".to_string();

    // 所有权转移到函数
    get_char(&data);
    // data 不再拥有所有权
    // 解决方案 ---> get_char 不转移所有权，只是 borrow

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// 应该获得所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
