use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String; // can have default implementation here
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn traits() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

}

pub fn _notify<T: Summary + Display>(item: &T) {
    println!("{}, {}", item.summarize(), item.to_string());
}