fn main() {
    let x = 5;
    // x = 6; // error occurs because x is immutable
    let mut y = "hello";
    println!("x is {x}, y is {y}");
    y = "hello world"; // ok
    println!("x is {x}, y is {y}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 61 * 3; // evaluate to 10800 during compilation
    let x = "5"; // shadowing
    println!("x is {x}, three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}
