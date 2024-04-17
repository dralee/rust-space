/**
 * (proc_macro)函数宏示例
 * 2024.04.17 by dralee
 */

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42} ".parse().unwrap()
}