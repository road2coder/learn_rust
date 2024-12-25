use std::collections::HashMap;

// For types that implement the Copy trait, like i32, the values are copied into the hash map.
// For owned values like String, the values will be moved and the hash map will be the owner of those values
fn main() {
    // create a hashmap
    let mut scores = HashMap::new();
    // insert a new item or update existed item
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{scores:?}");
    scores.insert(String::from("Yellow"), 99);
    // adding a key and value only if a key isn’t present
    scores.entry(String::from("Red")).or_insert(0);
    scores.entry(String::from("Yellow")).or_insert(0);

    // access values in a hashmap
    let score1 = scores.get("Blue").copied().unwrap_or(0);
    let score2 = scores.get("Red").copied().unwrap_or(0);
    println!("{score1}, {score2}");
    for (k, v) in &scores {
        println!("{k}, {v}");
    }
    // update values
    for (_, v) in &mut scores {
        *v -= 1;
    }
    println!("{scores:?}");
    // or
    for v in scores.values_mut() {
        *v -= 1;
    }
    println!("{scores:?}");
    // or
    scores.iter_mut().for_each(|(_, v)| {
        *v -= 1;
    });
    println!("{scores:?}");

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let map = text
        .split_whitespace()
        .fold(HashMap::<_, u32>::new(), |mut acc, word| {
            let count = acc.entry(word).or_insert(0);
            *count += 1;
            acc
        });
    println!("{map:?}");
}
