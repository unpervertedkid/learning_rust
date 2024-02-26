use fully_qualified_syntax_for_disambiguation::fly::{Human, Pilot, Wizard};
use fully_qualified_syntax_for_disambiguation::shelter::{Animal, Dog};

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
