use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
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
    // can specify a parameter to be something that implements a Trait
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound
// impl Trait is syntax sugar for the Trait Bound syntax <T: Summary>
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple traits syntax sugar
pub fn notify2(item: &(impl Summary + Display)) {}
// works for trait bound syntax too
pub fn notify3<T: Summary + Display>(item: &T) {}

// where clause
pub fn fn1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// use trait bound to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

// implemented for all T
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// only implemented if T has Display and PartialOrd trait
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementation using Trait Bound
// implement Summary trait for all any types that implements the Display trait
impl<T: Display> Summary for T {
    fn summarize(&self) -> String {
        format!("yaya")
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
}
