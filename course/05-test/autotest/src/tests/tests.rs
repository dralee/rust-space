/**
 * 自动化测试
 * 2024.03.29 by dralee
 */
fn greeting(name: &str) -> String {
	format!("Hello {}!", name)
}

pub struct Guess {
	value: i32,
}
impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}", value);
		}
		 Guess { value }
	}

	pub fn new2(value: i32) -> Guess {
		if value < 1 {
			panic!("Guess value must be greater than 1, got {}", value);
		}
		if value > 100 {
			panic!("Guess value must be less than 100, got {}", value);
		}

		 Guess { value }
	}
}

fn prints_and_returns_10(a: i32) -> i32 {
	println!("I got the value {}", a);
	10
}

#[cfg(test)]
pub mod test_demo{
	use super::*; // 使用上级空间
    //use crate::tests::tests::greeting;
	
	#[test]
	pub fn greeting_contains_name(){
		let result = greeting("Carol");
		assert!(result.contains("Carol"));
	}

	#[test]
	pub fn greeting_contains_name_not(){
		let result = greeting("World");
		assert!(
			result.contains("Carol"),
			"Greeting did not contain name, value was `{}`",
			result
		);
	}

	#[test]
	#[should_panic] // 必须位于#[test]之后,发生panic则通过
	fn greater_than_100(){
		Guess::new(120);
	}

	#[test]
	#[should_panic] // 必须位于#[test]之后,发生panic则通过,不发生panic则不通过
	fn less_than_100(){
		Guess::new(90);
	}

	#[test]
	#[should_panic(expected="less than or equal to 100")] // 必须位于#[test]之后,发生panic则通过,不发生panic则不通过
	fn greater_than_100_2(){ // panic消息为"Guess value must be less than 100, got 102",非"less than or equal to 100",所以结果Fail
		Guess::new2(102);
	}

	#[test]
	#[should_panic(expected="Guess value must be less than 100, got 102")] // 必须位于#[test]之后,发生panic则通过,不发生panic则不通过
	fn greater_than_100_2_ok(){ // panic消息为"Guess value must be less than 100, got 102",与should_painic的expected消息"Guess value must be less than 100, got 102"一致,所以结果ok
		Guess::new2(102);
	}

	// 返回结果: Ok及Err
	#[test]
	fn work_result() -> Result<(), String> {
		if 2 + 2 == 4 {
			Ok(())
		} else {
			Err(String::from("two plus two does not equal four"))
		}
	}
	// You can’t use the #[should_panic] annotation on tests that use Result<T, E>
	// Instead, use assert!(value.is_err())


	#[test]
	fn this_test_will_pass(){
		let value = prints_and_returns_10(5);
		assert_eq!(10, value);
	}

	#[test]
	fn this_test_will_fail(){
		let value = prints_and_returns_10(10);
		assert_eq!(5, value);
	}

	#[test]
	#[ignore] // 忽略本测试
	fn expensive_test(){
		println!("this is the exensive test, just ignore!");
		assert_eq!(9,3+3+3);
	}
}
