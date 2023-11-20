use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked_color())
    }

    fn most_stocked_color(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_1_preference = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_1_preference);
    println!(
        "The user with preference {:?} will get a {:?} shirt",
        user_1_preference, giveaway1
    );

    let user_2_preference = None;
    let giveaway2 = store.giveaway(user_2_preference);
    println!(
        "The user with preference {:?} will get a {:?} shirt",
        user_2_preference, giveaway2
    );

    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
