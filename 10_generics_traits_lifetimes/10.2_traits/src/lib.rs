#![allow(unused)]

use std::fmt::Display;
pub trait Summary {
    // without default implementation
    // fn summarize(&self) -> String;
    // with implementation
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

// traits as parameters
pub fn notify1(item: &impl Summary) -> String {
    item.summarize()
}
pub fn notify2(item: &(impl Summary + Display)) -> String {
    item.summarize()
}
pub fn notify3(item1: &impl Summary, item2: &impl Summary) -> String {
    item1.summarize() + &item2.summarize()
}

// trait bound
pub fn notify_trait_bound1<T: Summary>(item: &T) -> String {
    item.summarize()
}
pub fn notify_trait_bound2<T: Summary + Display>(item: &T) -> String {
    item.summarize()
}
pub fn notify_trait_bound3<T: Summary>(item1: &T, item2: &T) -> String {
    item1.summarize() + &item2.summarize()
}

// where clauses
pub fn notify_where_clause1<T>(item: &T) -> String
where
    T: Summary,
{
    item.summarize()
}
pub fn notify_where_clause2<T>(item: &T) -> String
where
    T: Summary + Display,
{
    item.summarize()
}
pub fn notify_where_clause3<T>(item1: &T, item2: &T) -> String
where
    T: Summary,
{
    item1.summarize() + &item2.summarize()
}

struct Pair<T> {
    x: T,
    y: T,
}
//  conditional implementation
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
// or
impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    // --snip --
}
