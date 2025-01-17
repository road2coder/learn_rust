#![allow(unused)]
fn main() {
    let csp1 = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let csp2 = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let csp = CustomSmartPointer {
        data: String::from("some data"),
    };
    // csp.drop(); // can not drop Drop::drop straightforwardly
    println!("csp created");
    drop(csp);
    println!("csp dropped");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}
