use std::cell::RefCell;

fn main() {
    let data = RefCell::new(10);

    // immutable
    println!("Final value: {}", data.borrow());
    // mutable
    {
        let mut borrow_mut = data.borrow_mut();
        *borrow_mut += 5;
        println!("Modified value: {}", *borrow_mut);
    }
    println!("Final value: {}", data.borrow());
}
