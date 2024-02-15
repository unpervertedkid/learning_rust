struct Value {
    value: f64,
}

struct GenericValue<T> {
    generic_value: T,
}

impl Value {
    fn value(&self) -> &f64 {
        &self.value
    }
}

impl<T> GenericValue<T> {
    fn value(&self) -> &T {
        &self.generic_value
    }
} 

fn main() {
    let x = Value {value: 3.0 };
    let y = GenericValue { generic_value: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
