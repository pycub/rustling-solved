#[derive(PartialEq, Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        return user_preference.unwrap_or_else(|| self.most_stocked());
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut reds = 0;
        let mut blues = 0;

        for shirt in self.shirts {
            match shirt {
                ShirtColor::Red => reds += 1,
                ShirtColor::Blue => blues += 1,
            }
        }

        if reds > blues {
            return ShirtColor::Red;
        }

        return ShirtColor::Blue;
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Red],
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
