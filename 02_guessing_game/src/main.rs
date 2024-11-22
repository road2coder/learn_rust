use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guessing the number!");
    let mut min = 1;
    let mut max = 100;
    let secret_number = rand::thread_rng().gen_range(min..max);

    loop {
        let mut guess = String::new();
        println!("Please input your guess({min} to {max}).");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                if secret_number > min {
                    min = guess
                }
            }
            Ordering::Greater => {
                if guess < max {
                    max = guess
                }
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
