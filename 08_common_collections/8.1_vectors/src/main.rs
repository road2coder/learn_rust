#![allow(unused)]
fn main() {
    // create a vector with vec! macro
    let mut v = vec![0, 1, 2, 3];
    println!("{v:?}");
    // add an element
    v.push(4);
    println!("{v:?}");
    // iterating over the values in a vector
    for i in &v {
        print!("{i} ");
    }
    println!();
    // update
    for i in &mut v {
        *i *= 2
    }
    println!("{v:?}");
    // access element by index
    let first = &v[0];
    println!("The first of element is {first}");
    let eighth = v.get(8);
    if let Some(i) = eighth {
        println!("The first of element is {i}");
    } else {
        println!("The first of element is None");
    }
    // error: index out of bounds
    // let eighth = &v[8];
    // println!("The first of element is {eighth}");
    // using an enum to store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.62),
    ];
    println!("{row:?}");
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
