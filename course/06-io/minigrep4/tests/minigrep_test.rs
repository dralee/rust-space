
use minigrep4;

#[test]
fn serach_text(){
	let query = "duct";
	let contents = "\
Rust:
safe, fast, productive.
Pick three.";
	assert_eq!(vec!["safe, fast, productive."], minigrep4::search(query, contents));
}

#[test]
fn case_insensitive(){
	let query = "rUst";
	let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
	assert_eq!(vec!["Rust:", "Trust me."], minigrep4::search_case_insensitive(query, contents))
}