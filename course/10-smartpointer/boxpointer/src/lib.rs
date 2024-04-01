use std::ops::Deref;

/**
 * 定义自定义智能指针
 * 2024.04.01 by dralee
 */

 pub struct MyBox<T>(T); // 定义元组结构,数据会存储在.0元素中

 impl <T> MyBox<T> {
	 pub fn new(x: T) -> MyBox<T> {
		MyBox(x)
	 }
 }

 /// 为了实现*y取值,需要实现Deref特征
 impl <T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0	
	}
 }