// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!
        // 使用旧的实例初始化新的结构体
        // 其中 name 和 count 的所有权都被转移了，
        // 其他的则是 copy trait
        // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
        // 任何一组简单标量值的组合都可以实现 Copy
        // 任何不需要分配内存或某种形式资源的类型都可以实现 Copy
        let your_order = Order {
          name: String::from("Hacker in Rust"),
          count: 1,
          ..order_template
        };
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);

        // 如果没有在新的结构体中重新指定 name 字段，
        // 那么原来的 String 所有权被转移给了新的结构体
        // 因此不能再使用原来的结构体中的 String

        // 如果我们重新指定了 name，那么原来的 String 所有权依旧在原来那里，没有被转移
        // assert_eq!(order_template.name, "Bob");
    }
}
