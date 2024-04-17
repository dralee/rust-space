/**
 * 测试自定义宏
 * 2024.04.17 by dralee
 */
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct TestA;

#[test]
fn test_macro(){
	TestA::hello_macro();
}