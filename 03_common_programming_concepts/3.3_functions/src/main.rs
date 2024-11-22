// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.
fn main() {
    // let x = (let y = 6); error because assign is a statement
    let x = 1;
    let y = plus_one(x);
    println!("The result of {x} plus one is {y}");
}

fn plus_one(i: i32) -> i32 {
    i + 1 //  return the last expression implicitly
}
