use std::fmt::Display;
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// If you want to use the default implementation you should us this
// impl Summary for NewArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// This is the same
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary)Este permite que sean de tipos diferentes mientras que ambos tengan el trait Summary
// pub fn notify<T: Summary>(item1: &T, item2: &T) Aquí se forza a que ambos sean del tipo T

// fn some_funtion<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// RETURNING TYPES THAT IMPLEMENT TRAITS
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from(" "),
        content: String::from(" "),
        reply: false,
        retweet: false,
    }
}
