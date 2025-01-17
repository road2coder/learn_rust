use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = Box::new(5);
    assert_eq!(5, *x);

    let x = MyBox::new(5);
    assert_eq!(5, *x);
    let y = *x;
    // actually ran this code
    // let y = *x.deref();
    assert_eq!(5, y);

    let name = MyBox::new(String::from("Rust"));
    // implicit deref coercion
    hello(&name);
    // equals to
    hello(&(*name)[..]);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {name}");
}
