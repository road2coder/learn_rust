#![allow(unused)]
use garden::vegetables::{self, Asparagus};

// src/garden.rs or src/garden/mod.rs
// or within curly brackets
// mod garden {}
mod garden;

fn main() {
    garden::fn_in_garden();
    vegetables::fn_in_vagetables();
    let plant = Asparagus {};
    println!("I'm growing {plant:?}");
}
