use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5);
    println!("i32 in Box: {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// recursive type `List1` has infinite size [E0072]
// enum List1<T> {
//     Cons(T, List1<T>),
//     Nil
// }
