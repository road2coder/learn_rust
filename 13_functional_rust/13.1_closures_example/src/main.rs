fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

// #[derive(Debug, Copy)]
#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // unwrap_or -> most_stocked will be called multiple times
        // user_preference.unwrap_or(self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        println!("most_stocked");
        let (num_red, num_blue) = self.shirts.iter().fold((0_u32, 0_u32), |mut acc, color| {
            match color {
                ShirtColor::Red => acc.0 += 1,
                ShirtColor::Blue => acc.1 += 1,
            }
            acc
        });
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
