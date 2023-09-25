enum IpAddress {
    V4(u8,u8,u8,u8),
    V6(String),
}

//If you dont speak swahili dont worry too much about these, they're just swahili slang for money
enum Pesa {
    Mbao,
    Chwani,
    Soo,
    Punch,
    Thao,
    Mita,
}
fn main() {
    let home = IpAddress::V4(127,0,0,1);
    let loopback = IpAddress::V6(String::from("::1"));

    let my_money = Pesa::Thao;
    let my_money_in_shillings = value_in_shillings(my_money);
    println!("I have {} shillings", my_money_in_shillings );

    //Let me try and double my money 😎
    let double_my_money_maybe = double_my_money(Some(my_money_in_shillings));

    if let Some(money) = double_my_money_maybe {
        println!("I have double my money: {} 🥳", money);
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
    match maybe_some_money {
        None => None,
        Some(money) => Some(money * 2)
    }
}
