fn main() {
    // scope and assignment
    // { // s is not valid here
    //     let s = "inner hello"; // s in valid
    //     println!("{s}");
    // } // s in no longer valid
    // String can be mutated
    let mut s = String::from("hello");
    s.push_str(", mutable string");
    println!("{s}");
    // copy (all scalar types, specific tuples and arrays)
    let x = 5;
    let y = x;
    println!("x is {x} and y is {y}");
    // move
    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("s1 is {s1} and s2 is {_s2}"); error: moved s1 can not be used anymore

    // ownership and functions
    let s = String::from("hello ");
    takes_ownership(s); // s's value moves in to fn,
                        // and so it is no longer valid here
    let x = 5;
    maske_copy(x); // i32 is Copy, it's ok to use x afterward
    let s1 = gives_ownership();
    let s2 = takes_and_gives_back("hello_world".to_string());
    println!("{s1}, {s2}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn maske_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
