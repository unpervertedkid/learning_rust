fn main() {
    // Creating a string
    // Using to_string()
    let mut _string = String::new();
    let data = "initial contents";
    _string = data.to_string();

    drop(_string);
    // Using String::from()
   let mut _string = String::from("initial contents");
   drop(_string);

   // Updating a String
   let mut _string = String::from("foo");
   let string_2 = "bar";

   _string.push_str(string_2);
drop(_string);

   // Indexing into strings
   let _string = String::from("Hello World");

   // Slicing strings
   let _world = &_string[6..];

   // Iterating over String
   for character in _string.chars() {
       println!("{character}");
   }
}
