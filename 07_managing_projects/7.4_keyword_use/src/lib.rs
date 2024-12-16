#![allow(unused)]

// external crate
use rand::Rng;
// rename
use std::fmt::Result;
use std::io::Result as IoResult;
// nested paths
use std::{cmp::Ordering, io};
// to bring all public items defined in std::collections
use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Bringing Paths into Scope with the use Keyword
// use crate::front_of_house::hosting;

// Re-exporting Names with pub use
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant() {
        // error, not the same scope with hosting
        // hosting::add_to_waitlist();
        // ok
        super::hosting::add_to_waitlist();
    }
}
