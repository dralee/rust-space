/**
 * grep之mini版功能-业务部分
 * 2024.04.01 by dralee
 */
use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool, // 增加忽略大小写
}

impl Config {
    // 一旦 Config: : build 获得迭代器的所有权并停止使用借用的索引操作，我们就可以将 String 值从迭代器移动到 Config 中，而不是调用 clone 并进行新的分配。
    // 解决返回值:clone问题
    pub fn build(mut args: impl Iterator<Item=String>,) -> Result<Config,&'static str> {
        args.next();  // 去除掉args[0]

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // 从环境变量中获取是否忽略大小写
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;    
    //println!("with text:\n{contents}");
    // for line in search(&config.query, &contents) {
    //     println!("{line}");
    // }
    // 增加支持大小写指定
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// 大小写匹配查询
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 忽略大小写查询
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    // results
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}