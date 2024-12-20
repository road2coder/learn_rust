#![allow(unused)]

// Strings are utf-8 encoded
// Strings are not so simple

fn main() {
    // create a string "hello world"
    let mut s = String::new();
    s.push_str("hello world");
    let s = "hello world".to_string();
    let s = String::from("hello world");
    println!("{s}");

    // "hello" in different languages
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");
    s.push_str(" foo");
    println!("{s}");

    // the "+" operator
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s = s1 + &s2; // s1 has been moved
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // no one has been moved
    println!("{s}");

    // error: can not access characters in a string by index
    // let s1 = String::from("hello");
    // let h = s1[0];

    let s = String::from("नमस्ते");
    println!("{:?}", s.as_bytes());
    println!("{:?}", s.chars());

    // let hello = "Здравствуйте";
    // let h = &s[0..4]; //panic:  byte index 4 is not a char boundary; it is inside 'म' (bytes 3..6) of `नमस्ते`
    // println!("{s}");

    // iterate over strings
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
