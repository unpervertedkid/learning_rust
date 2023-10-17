pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Once upon a time there existed a crab. He was called a rustacean...");
    let first_sentence = novel.split('.').next().expect("Could not find a .");
    let i = ImportantExcerpt {
        part : first_sentence,
    };
    println!("The first sentence is: {}" , i.part); 
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}