fn main() {
    let v1 = [1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = [1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn consume_produce_iterator() {
    let v = [1, 2, 3, 4, 5];
    let v1_iter = v.iter();
    // sum consume the iterator so that v1_iter will have been moved
    let total1: i32 = v1_iter.sum();
    // map produce a iterator
    let total2: i32 = v.into_iter().map(|x| x * 2).sum();
    assert_eq!(total1 * 2, total2);
}
