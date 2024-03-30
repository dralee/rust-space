/**
 * grep之mini版功能
 * 2024.03.30 by dralee
 */
use std::env;
use std::process;
use minigrep2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let (query, file_path) = parse_config(&args);
    //let config = parse_config(&args);
    //let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // 6.业务抽离main
    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");
    // println!("poem:\n{contents}");
    //run(config);
    if let Err(e) = minigrep2::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// 6.业务抽离main
// fn run(config: Config) {
//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");
//     println!("poem:\n{contents}");
// }

// 7.解决异常问题
/* 8.移动到lib.rs 减少main.rs业务代码
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;    
    println!("with text:\n{contents}");

    Ok(())
}*/

// 8.移动到lib.rs 减少main.rs业务代码
// struct Config {
//     query: String,
//     file_path: String
// }

// 1.由main直接使用,抽取为函数
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];
    
//     (query, file_path)
// }

// 2.由直接返回tuple抽取为函数返回对象
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
    
//     Config {query, file_path}
// }

// 3.进一步直接抽取为类方法 
/* 8.移动到lib.rs 减少main.rs业务代码
impl Config {
    // fn new(args: &[String]) -> Config {
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Config { query, file_path }
    // }
    // 4.解决索引出界问题
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Config { query, file_path }
    // }
    // 5.索引出界问题,返回Result
    fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}*/

