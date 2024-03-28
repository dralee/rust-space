/**
 * 重复代码去除,泛型/生命周期
 * 2024.03.28 by dralee
 */
mod generic;
pub use crate::generic::generic_type;

mod traits;
use crate::trait_type::NewsArticle;
pub use crate::traits::trait_type::{self, Summary, Tweet, notify};

fn main() {
    generic();
    
    traits();
}

fn generic(){
    generic_type::duplication_demo();
    generic_type::remove_duplication_demo();

    generic_type::generic_fn();
}

fn traits(){
    trait_type::hello();

    let tweet = Tweet {
        username: String::from("hourse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet default: {}", tweet.short_summarize());
    println!("the new tweet author: {}", tweet.summarize_author());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.")
    };
    println!("New article available! {}\n{}", article.summarize(), article.short_summarize());
    println!("the article author: {}", article.summarize_author());

    println!("=================");
    notify(&tweet);
    notify(&article);
}

