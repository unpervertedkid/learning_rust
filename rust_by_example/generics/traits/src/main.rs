struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self,_:T);
}

impl<T,U> DoubleDrop<T> for U {
    fn double_drop(self,_:T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    //empty;
    //null;
    //
    //Uncommenting above lines would cause a compilation error since both have been dropped.
}
