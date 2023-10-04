use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let top_secret_file_result = File::open("top_secret.txt");

    let _top_secret_file = match top_secret_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("top_secret.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
            
        }
        };

        // Alternatives to using match to handle different errors
        let top_secret_file = File::open("top_secret.txt").expect("Failed to open the top secret file");

        let top_secret_file_title = read_top_secret_title_from_file().expect("Title should be present");
        println!("The top secret title is: {}", &top_secret_file_title)
}

//Propagating errors
fn read_top_secret_title_from_file() -> Result<String, io::Error> {
    let top_secret_file_result = File::open("top_secret.txt");

    let mut top_secret_file = match top_secret_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut top_secret_title = String::new();

    match top_secret_file.read_to_string(&mut top_secret_title) {
        Ok(_) => Ok(top_secret_title),
        Err(error) => Err(error),
    }
}
