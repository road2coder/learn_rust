#![allow(unused)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle::from(30, 20);
        let rect2 = Rectangle::from(19, 29);
        assert!(rect1.can_hold(&rect2))
    }

    #[test]
    #[should_panic]
    fn panic_test() {
        might_panic(true);
    }

    #[test]
    #[should_panic(expected = "Got true")]
    fn panic_test_with_msg() {
        might_panic(true);
    }

    #[test]
    fn test_return_result() -> Result<(), &'static str> {
        Ok(()) // success test

        // Err("test failed") // filed test
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn from(size1: u32, size2: u32) -> Rectangle {
        let width = if size1 > size2 { size1 } else { size2 };
        let height = size1 + size2 - width;
        Rectangle { width, height }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn might_panic(is_panic: bool) {
    if (is_panic) {
        panic!("Got true")
    }
}
