use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::List::{Cons, Nil};
    #[test]
    fn multiple_owner_mutable() {
        let value = Rc::new(RefCell::new(String::from("init")));
        // let a = Rc::new(Cons(Rc::clone(value), Rc::new(Nil));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(String::from("b"))), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(String::from("c"))), Rc::clone(&a));

        println!("===========before=========");
        println!("a = {a:?}");
        println!("b = {b:?}");
        println!("c = {c:?}");
        value.borrow_mut().push_str(" add something");
        println!("===========after=========");
        println!("a = {a:?}");
        println!("b = {b:?}");
        println!("c = {c:?}");
    }
}
