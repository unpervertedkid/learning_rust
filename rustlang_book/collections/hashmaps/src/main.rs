use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();


    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 25);

    let team_name = String::from("Red");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}: {}", team_name, score);


    for (team, score ) in &scores {
        println!("{}: {}", team, score);
    }
}
