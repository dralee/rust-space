use integration;

mod common;

#[test]
fn test_add(){
	common::setup();
	
	let result = integration::add_two(2);
	assert_eq!(result, 4);
}