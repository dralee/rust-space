/**
 * 派生宏示例(derive_macro)
 * 2024.04.17 by dralee
 */

extern crate demoderive;
use demoderive::AnswerFn;

#[derive(AnswerFn)]
struct Struct;

fn main() {
    assert_eq!(42, answer());
}
