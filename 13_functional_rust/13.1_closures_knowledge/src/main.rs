#![allow(unused)]
fn main() {
    let mut list = vec![1, 2, 3];

    // Fn
    let only_borrows = || {
        println!("from closure: {list:?}");
    };
    println!("before calling only_borrows: {list:?}");
    only_borrows();
    println!("after calling only_borrows: {list:?}");
    // FnMut
    let mut borrows_mutably = || list.push(list.len() + 1);
    borrows_mutably();
    borrows_mutably();
    borrows_mutably();
    println!("after calling borrows_mutably 3 times: {list:?}");
    // FnOnce
    let takes = || list;
    let new_list = takes();
    println!("list has been moved to new list: {new_list:?}");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut num_closure_called = 0;
    // FnMut
    list.sort_by_key(|r| {
        num_closure_called += 1;
        r.width
    });
    println!("{list:#?}");
    println!("num_closure_called: {num_closure_called}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
