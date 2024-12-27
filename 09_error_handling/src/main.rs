#![allow(unused)]
use std::fs;

// https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
fn main() {
    println!("Hello, world!");
    // recoverable errors with Result: std::fs::read_to_string
    let s = match fs::read_to_string("hello.txt") {
        Ok(s) => s,
        _ => panic!("Problem reading hello.txt to string."),
    };
}

// Unrecoverable Errors with panic!
fn will_panic() {
    panic!("Unrecoverable error occurred");
}
