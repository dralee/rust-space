/**
 * 示例宏
 * 2024.04.17 by dralee
 */

// 定义`helper_macro`箱
#[macro_export]
macro_rules! helped {
    () => {
        $crate::helper!()
    }
}

#[macro_export]
macro_rules! helper {
    () => { () };
}

// 其他crate中使用helped，需要引入
//use helper_macro::helped;

pub fn unit(){
	helped!();
}

pub mod inner {
	#[macro_export]
	macro_rules! call_foo {
		() => {
			$crate::inner::foo()
		};
	}

	pub fn foo() {
		println!("inner foo called.");
	}
}

