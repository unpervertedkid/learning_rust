fn main() {
    println!("10 + 1 = {}", add_one(Some(10)).unwrap());

    separator();
    let is_raining = Some(true);
    let preferred_color = Some("Black");

    get_dress_code(&is_raining, preferred_color);

    separator();
    let mut tasks = vec!["Wake up", "Plan my day", "Read a chapter from the book how to be bored", "Do one chapter from the rust book"];

    task_manager(&mut tasks);

    separator();
    let shopping_list = vec!["Tomatoes", "Eggs", "Toiletries", "Wet wipes", "Avocadoes"];
    print_shopping_list(&shopping_list);
}


// Match arms
fn add_one(input: Option<i32>) -> Option<i32> {
    match input {
    None => None,
    Some(number) => Some(number + 1),
    }
}

// If let expressions
fn get_dress_code(is_raining: &Option<bool>, preferred_color: Option<&str>) {
    if let Some(color) = preferred_color {
        println!("You should wear {} clothes", color);
    } else if let Some(_) = is_raining {
        println!("Dont forget to carry your umbrella as it will rain today");
    }
}

// While let loops
fn task_manager(tasks: &mut Vec<&str>) {
    while let Some(task) = tasks.pop() {
        println!("Doing task {}", task);
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Done ✅");
    }
}

fn print_shopping_list(list: &Vec<&str>) {
    println!("Shopping List");
    separator();

   for (index, item) in list.iter().enumerate() {
        println!("{}: {}", index + 1, item);
    }
}

fn separator() {
    println!("----------------------------------");
}
