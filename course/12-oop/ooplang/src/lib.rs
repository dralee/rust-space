/**
 * 面向对象语言特征
 * 2024.04.10 by dralee
 */

 pub struct AveragedCollection {
	list: Vec<i32>,
	average: f64,
 }

 impl AveragedCollection {
	pub fn new() -> AveragedCollection {
		AveragedCollection {
			list: vec![],
			average: 0.0
		}
	}

	 pub fn add(&mut self, value: i32) {
		self.list.push(value);
		self.update_average();
	 }

	 pub fn remove(&mut self) -> Option<i32> {
		let result = self.list.pop();
		match result {
			Some(value) => {
				self.update_average();
				Some(value)
			}
			None => None,
		}
	 }

	 pub fn average(&self) -> f64 {
		self.average
	 }

	 fn update_average(&mut self) {
		let total: i32 = self.list.iter().sum();
		self.average = total as f64 / self.list.len() as f64;
	 }
 }


 pub trait Draw {
	 fn draw(&self);
 }

 // 特征对象
pub struct Screen {
	 pub components: Vec<Box<dyn Draw>>, // dyn Draw指定为实现了Draw特征的类型，行为特征
 }
 impl Screen {
	 pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	 }
 }

 // 使用泛型类型和特征绑定定义screen结构体
 // !!!泛型类型参数一次只能替换为一种具体类型，而特征对象允许在运行时填充多个具体类型的特征对象
 pub struct Screen2<T: Draw> {
	pub components: Vec<T>,
 }
 impl <T> Screen2<T>
 where T:Draw, {
	pub fn run(&self){
		for component in self.components.iter(){
			component.draw();
		}
	}
 }

 pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
 }
 impl Draw for Button {
	 fn draw(&self) {
		 println!("The button({},{}) of {} is drew!", self.width, self.height, self.label);
	 }
 }

 pub struct SelectBox {
	pub width: u32,
	pub height: u32,
	pub options: Vec<String>,
 }
 impl Draw for SelectBox {
	 fn draw(&self) {
		println!("The selectBox({},{}) for {:?} is drew!",self.width,self.height,self.options);
	 }
 }