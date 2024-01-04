use rand::Rng;
use std::ops::Range;
use enums::ip_address::{IpAddress};

fn main() {
  let home = IpAddress::V4(127,0,0,1);

    println!("{:?}", home);

    let my_money = Pesa::Thao;
    let my_money_in_shillings = value_in_shillings(my_money);
    println!("I have {} shillings", my_money_in_shillings );

    //Let me try and double my money 😎. Its risky though, I might lose it all 😭
    println!("Let me try and double my money 😎. Its risky though, I might lose it all 😭");
    let double_my_money_maybe = double_my_money(Some(my_money_in_shillings));

    if let Some(money) = double_my_money_maybe {
        println!("I have double my money🥳\nI now have {} shillings🤑", money);
    } else {
        println!("I lost all my money 😭");
    }
}

fn value_in_shillings(pesa: Pesa) -> u64 {
    match pesa {
        Pesa::Mbao => 20,
        Pesa::Chwani => 50,
        Pesa::Soo => 100,
        Pesa::Punch => 500,
        Pesa::Thao => 1000,
        Pesa::Mita => 1000000,
    }
}

fn double_my_money(maybe_some_money: Option<u64>) -> Option<u64> {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(Range { start: 0, end: 10 });

    match maybe_some_money {
        None => None,
        Some(money) => {
            if random_number < 3 {
                None
            } else {
                Some(money * 2)
            }
        }
    }
}

enum Pesa {
    Mbao,
    Chwani,
    Soo,
    Punch,
    Thao,
    Mita,
}
