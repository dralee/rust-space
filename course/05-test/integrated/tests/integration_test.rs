/**
 * 多测试方法集成测试
 * 2024.03.30 by dralee
 */
use integrated;

// 不再需要#[cfg(test)]
#[test]
fn it_adds_two() {
	assert_eq!(4, integrated::add_two(2));
}