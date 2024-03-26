// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
/**
 * 模块lib
 * functions, methods, structs, enums, modules, and constants默认都是private的
 * 父模块中的项不能使用子模块中的私有项，但子模块中的项可以使用其祖先模块中的项
 * struct为pub,而其内字段未显式标为pub,则默认为private;
 * enums为pub,则其内字段全为pub
 * 2024.03.26 by dralee
 */

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}

        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

fn deliver_order() {}

mod back_of_house {
    use core::str;

    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order(); // 调用父中的方法
    }

    fn cook_order(){}

    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant(){
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // 修改为喜欢的吐丝
    println!("I'd like {} toast please", meal.toast);
    //meal.seasonal_fruit = String::from("blueberries"); // 无法编译,private的不允许访问

    let order1 = back_of_house::Appetizer::Soup; // 可访问,级别为pub
    let order2 = back_of_house::Appetizer::Salad;
}

use crate::front_of_house::hosting; // 引入
//use crate::front_of_house::hosting::add_to_waitlist; // 习惯性使用parent.xxx,而不直接引入到fn级,避免与本地函数混淆
// 引用类型struct/enums,则使用全路径到类型
pub fn eat_at_restaurant2(){
    hosting::add_to_waitlist(); // 引入后,可直接使用其下内容
}

mod customer {
    use crate::front_of_house::hosting;
    use std::collections::HashMap;

    pub fn eat_at_restaurant(){
        hosting::add_to_waitlist();
    }

    fn demo(){
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
}

mod same_name{
    use std::fmt;
    use std::io;
    use std::io::Result as IoResult; // 重命名

    // fn function1() -> fmt::Result {

    // }

    // fn function2() -> io::Result<()> {
        
    // }

    // fn function2() -> IoResult<()> {
        
    // }
}

//当我们使用 use 关键字将名称引入作用域时，新作用域中的名称是私有的。
//为了使调用我们代码的代码能引用该名称，就好像它是在该代码的作用域中定义的一样，
//我们可以将 pub 和 use 结合起来。这种技术被称为再导出，因为我们在将一个项目引入作用域的同时，
//也让其他人可以将该项目引入他们的作用域。
mod use_pub_mod {
    pub use crate::front_of_house::hosting;
    pub fn eat_at_restaurant(){
        hosting::add_to_waitlist();
    }
}