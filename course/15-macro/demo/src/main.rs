/**
 * 示例宏
 * 2024.04.17 by dralee
 */

macro_rules! foo {
    ($l:tt) => {
        bar!($l);
    };
}

macro_rules! bar {
    (3) => { 
        println!("bar!");
    };
}

//use helper_macro::helped;

use demo::unit;

fn main() {
    foo!(3);

    unit();
}