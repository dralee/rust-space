/*
 * pub use重新导出项目
 * by dralee 2024.04.01
 * 
*/
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
	let yellow = PrimaryColor::Yellow;

	let mx = mix(red, yellow);	
	println!("{:?}", mx);
}
