fn main() {
    let first_message = "Hello ";
    let second_message = "Rustaceans";

    let string_one = String::from(first_message);
    let string_two = second_message.to_string();


    let final_string = string_one + &string_two;
    println!("Final string: {}", final_string);

    for character in final_string.chars() {
        println!("{}", character);
    }
}
