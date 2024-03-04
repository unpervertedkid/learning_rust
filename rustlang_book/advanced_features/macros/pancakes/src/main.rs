use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Chapatis;

fn main() {
    Chapatis::hello_macro();
}
