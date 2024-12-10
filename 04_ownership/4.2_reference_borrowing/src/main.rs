// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.

fn main() {
    // reference
    let s = String::from("hello world");
    let len = calc_length(&s);
    println!("The length of {s} is {len}");

    // mutable reference (only can borrow from mutable variable)
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
    let mut i = 188;
    let r_i = &mut i;
    *r_i = 200;
    println!("i has been changed: {i}");
}

// the parameter s is a reference to a string and doesn't have ownership
fn calc_length(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// error: dangling references (references must always be valid)
// fn dangle() -> &String {
//     let s = String::from("hello world");
//     &s
// }
