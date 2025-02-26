#![allow(unused)]
fn main() {
    let x = Some(1);
    if let Some(y) = x {
        println!("{y} in Some({y})");
    }
    println!("{x:?}"); // ok

    let x = Some(String::from("hello"));
    if let Some(y) = x {
        println!("{y} in Some({y})");
    }
    // println!("{x:?}"); // error

    // named value
    let x = Some(5);
    match x {
        Some(y) => println!("x and y are both {y}"),
        Some(50) => println!("got 50"),
        _ => println!("default, x = {x:?}"),
    }
    // multiple
    let x = 4;
    match x {
        4 | 5 => println!("x is 4 or 5"),
        _ => println!("other"),
    }
    // ranges
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("{p:?}");
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // enum
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
}

// ignore value with _
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
