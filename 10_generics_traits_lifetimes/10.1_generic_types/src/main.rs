#![allow(unused)]

fn main() {
    let nums = vec![1, 2, 3, 5, 4, 3, 2, 4];
    let chars = vec!['c', 'd', 'a', 'b'];
    println!("{}", largest_i32(&nums).eq(largest(&nums)));
    println!("{}", largest_char(&chars).eq(largest(&chars)));

    let int_point = Point { x: 18, y: 32 };
    let float_point = Point { x: 18.0, y: 32.0 };
    println!("{}", int_point.special_method()); // ok
    // println!("{}", float_point.special_method()); // error
}

fn largest_i32(list: &[i32]) -> &i32 {
    if list.is_empty() {
        panic!("empty list");
    };
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    if list.is_empty() {
        panic!("empty list");
    };
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    if list.is_empty() {
        panic!("empty list");
    };
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn some_method(&self) -> &T {
        todo!()
    }
}

impl Point<i32> {
    fn special_method(&self) -> i32 {
        todo!()
    }
}

enum MyOption<T> {
    Some(T),
    None,
}

enum MyResult<T, U> {
    Ok(T),
    Err(U),
}
