/**
 * 面向对象设计实现
 * 2024.04.14 by dralee
 */
pub struct Post {    
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingPreivewPost {
        PendingPreivewPost {
            content: self.content,
        }
    }
}

pub struct PendingPreivewPost {
    content: String,
}

impl PendingPreivewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post:&'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingPreivew{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingPreivew {}
impl State for PendingPreivew {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Published{}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post:&'a Post) -> &'a str {
        &post.content
    }
}