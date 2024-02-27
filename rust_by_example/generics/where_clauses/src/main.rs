use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}
fn main() {
    let vector = vec![1, 2, 3, 4, 5];

    vector.print_in_option();
}
