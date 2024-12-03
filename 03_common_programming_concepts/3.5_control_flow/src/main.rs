fn main() {
    // if expressions (condition must be a bool)
    let number = 2;
    if number > 3 {
        println!("{number} is grater than 3")
    } else {
        println!("{number} is less than 3")
    };
    // if expression: expressions return a value
    println!(
        "{number} is {} than 3",
        if number > 3 { "grater" } else { "less" }
    );
    // multiple conditions
    let score: u32 = 60;
    if score >= 90 {
        println!("outstanding");
    } else if score >= 60 {
        println!("passed");
    } else {
        println!("failed");
    }

    // repetions with loops
    let mut i = 3;
    loop {
        if !(i > 0) {
            break;
        }
        println!("{i}");
        i -= 1;
    }
    // returning value from loops
    let mut count = 0;
    let res = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("result from loops: {res}");
    // loop label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut num = 3;
    while num != 0 {
        print!("{num}");
        num -= 1;
    }
    println!("LIFTOFF");

    // collection
    let nums = [10, 20, 30, 40, 50];
    for num in nums {
        println!("the value is {num}");
    }
    for num in (1..=3).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF");
}
