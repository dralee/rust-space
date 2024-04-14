/**
 * 面向对象设计模式
 * 2024.04.14 by dralee
 */
fn main() {
    test_post();
}

use ooppattern2::Post;

fn test_post() {
    let mut post = Post::new();

    post.add_text("I ate a saled for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a saled for lunch today", post.content());
}