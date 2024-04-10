/**
 * 面向对象设计模式
 * 2024.04.10 by dralee
 */
fn main() {
    test_post();
}

use ooppattern::Post;

fn test_post() {
    let mut post = Post::new();

    post.add_text("I ate a saled for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a saled for lunch today", post.content());
}