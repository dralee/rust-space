/*
 * 文档化注释测试
 * by dralee 2024.04.01
 * 
*/

//! tests
//! 多个模块混合测试
//! doclib & art

use doclib::kinds::PrimaryColor;
use doclib::kinds::SecondaryColor;
use doclib::utils::mix;

#[test]
fn test_art(){
	let red = PrimaryColor::Red;
	let yellow = PrimaryColor::Yellow;

	let mx = mix(red, yellow);
	//assert_eq!(mx, SecondaryColor::Orange); // binary operation `==` cannot be applied to type `SecondaryColor`
	println!("{:?}", mx); // cargo test -- --show-output
}