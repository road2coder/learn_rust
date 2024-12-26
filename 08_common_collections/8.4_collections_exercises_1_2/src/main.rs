use std::collections::{HashMap, HashSet};

fn main() {
    let (median, mode) = median_and_mode(&[1, 2, 2, 2, 5, 5, 6, 7, 8, 9]).unwrap();
    println!("{}, {}", median.eq(&5), mode.eq(&2));
    let should_be_none = median_and_mode(&[]);
    println!("{}", should_be_none.eq(&None));

    println!("{}", to_pig_latin("first").eq("irst-fay"));
    println!("{}", to_pig_latin("apple").eq("apple-hay"));
    println!("{}", to_pig_latin("first apple").eq("irst-fay apple-hay"));
    println!(
        "{}",
        to_pig_latin("  first apple    你好").eq("  irst-fay apple-hay    好-你ay")
    );
    println!("{}", to_pig_latin("Hello world, this is a test"));
}

fn median_and_mode(nums: &[i32]) -> Option<(i32, i32)> {
    if nums.is_empty() {
        return None;
    }
    let mut nums = nums.to_owned();
    nums.sort();
    let mut max_time: u32 = 0;
    let mode = nums
        .iter()
        .fold(HashMap::<_, u32>::new(), |mut acc, time| {
            let v = acc.entry(time).or_insert(0);
            *v += 1;
            acc
        })
        .iter()
        .fold(nums[0], |acc, (num, time)| {
            if *time > max_time {
                max_time = *time;
                **num
            } else {
                acc
            }
        });
    Some((nums[nums.len() / 2], mode))
}

fn to_pig_latin(s: &str) -> String {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    // 1. split s to Vector<str> by whitespace
    let mut strings = Vec::new();
    if !s.is_empty() {
        let mut prev_e: usize = 0; // the previous end position
        let mut prev_is_sp = (*s.as_bytes().first().unwrap() as char).is_whitespace();
        for (i, b) in s.as_bytes().iter().enumerate() {
            let is_sp = (*b as char).is_whitespace();
            if is_sp.ne(&prev_is_sp) {
                strings.push(&s[prev_e..i]);
                prev_e = i;
            }
            prev_is_sp = is_sp;
        }
        strings.push(&s[prev_e..]);
    }
    // 2. convert each word to pig latin
    strings.iter().fold(String::new(), |mut acc, &item| {
        let is_sp = (*item.as_bytes().first().unwrap() as char).is_whitespace();
        if is_sp {
            acc.push_str(item);
        } else {
            let mut owned = item.to_string();
            if vowels.contains(&item.chars().next().unwrap()) {
                owned.push_str("-hay");
            } else {
                let first = owned.remove(0);
                owned.push_str(&format!("-{first}ay"));
            }
            acc.push_str(&owned);
        };
        acc
    })
}
