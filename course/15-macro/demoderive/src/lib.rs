/**
 * 派生宏示例(derive_macro)
 * 2024.04.17 by dralee
 */
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
	"fn answer() -> u32 { 42 }".parse().unwrap()
}