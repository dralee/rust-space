/**
 * 属性宏
 *  属性宏定义了可附加到项的新外部属性，包括块中extern项、固有特征实现及特征定义。
 * 2024.04.18 by dralee
 */
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
	println!("attr: \"{}\"", attr.to_string());
	println!("item: \"{}\"", item.to_string());
	item
}