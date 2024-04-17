/**
 * 高级-自定义宏-实现
 * 使用macro_rules!规则声明宏、及3种过程宏：
 *  >自定义#[derive]达观，用于指定使用结构和枚举上使用的derive属性添加代码；
 *  >类似属性的宏，用于定义可用于任何项的自定义属性；
 *  >类似函数的宏，看起来像函数调用，但对指定为其参数的标记进行操作
 * 2024.04.16 by dralee
 */
extern crate proc_macro;
use proc_macro::TokenStream; // proc_macro来自Rust,因此不需要依赖，是编译器的API，允许从代码中读取和操作Rust代码
use quote::quote; // quote将数据结构转换为syn返回Rust代码
use syn; // syn将Rust代码从字符串解析为可执行操作的数据结构

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construcct a representation of Rust code as a syntax tree that we can manipulate
    // 可以语法树形式表示构建Rust代码
    let ast = syn::parse(input).unwrap();

    // 构造特征实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}