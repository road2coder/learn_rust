// a slice are a kind of reference that doesn't have ownership
// slices reference to a contiguous of elements in a collection
fn main() {
    // String slices
    let s = String::from("hello slices");
    let s_part1 = &s[..];
    println!("{}", s_part1.eq("hello slices"));
    let s_part2 = &s[6..];
    println!("{}", s_part2.eq("slices"));
    let s_part3 = &s[0..=4];
    println!("{}", s_part3.eq("hello"));
    // &String is &str too
    let s_full: &str = &s;
    println!("{s_full}");
    let s = "hello world";
    println!("the first world of \"{s}\" is {}", first_world(s));
    let s = "line1
helloworld";
    println!("the first world of \"{s}\" is {}", first_world(s));

    // other slices
    let nums = [0, 1, 2, 3, 4, 5];
    let nums_part1 = &nums[..3];
    println!("{}", nums_part1.eq(&[0, 1, 2]));
}

// &str is better than &String as a parameter
fn first_world(s: &str) -> &str {
    let whitespaces = [b' ', b'\t', b'\r', b'\n'];
    for (i, item) in s.as_bytes().iter().enumerate() {
        if whitespaces.contains(item) {
            return &s[..i];
        }
    }
    s
}
