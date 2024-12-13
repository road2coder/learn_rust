#![allow(unused)]
fn main() {
    let user = User {
        active: true,
        username: "Bob".to_owned(),
        email: String::from("bob@gmail.com"),
        age: 18,
    };
    println!("{user:#?}");
    // mutable instance
    let mut user = User {
        active: true,
        username: "Bob".to_owned(),
        email: String::from("bob@gmail.com"),
        age: 18,
    };
    user.age = 20;
    println!("{user:#?}");

    let rect1 = Rectangle::build(10, 20);
    println!(
        "The width of {rect1:?} is {} and the height of it is {}",
        rect1.width, rect1.height
    );
    println!("The area of {rect1:?} is {}", rect1.area());
    println!("{:?} is a square", Rectangle::square(30));
    let rect2 = Rectangle::build(17, 9);
    println!(
        "Can {rect1:?} can hold {rect2:?}?  {}",
        if rect1.can_hold(&rect2) { "Yes" } else { "No" }
    );
}

#[derive(Debug)] // implement Debug trait automatically
struct User {
    active: bool,
    username: String,
    email: String,
    age: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated function, similar to static function in other language
    fn build(size1: u32, size2: u32) -> Rectangle {
        let width = if size1 > size2 { size1 } else { size2 };
        let height = size1 + size2 - width;
        Rectangle { width, height }
    }
    // another associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    // method(the first parameter must be self)(&self -> self: &Self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// multiple impl Blocks
impl Rectangle {
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}
