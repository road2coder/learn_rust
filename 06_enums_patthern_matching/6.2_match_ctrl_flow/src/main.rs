#![allow(unused)]
fn main() {
    println!("Hello, world!");
    let coin = Coin::Penny;
    println!("The value of {coin:?} is {}", coin.value());
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter(_) => 25,
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
    // if let Some(i) = x {
    //     Some(i + 1)
    // } else {
    //     None
    // }
}
