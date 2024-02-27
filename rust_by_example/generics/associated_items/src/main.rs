struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.first() == number_1) && (&self.last() == number_2)
    }

    fn first(&self) -> Self::A {
        self.0
    }

    fn last(&self) -> Self::B {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 2;
    let number_2 = 4;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number is {}", container.first());
    println!("Last number is {}", container.last());

    println!("The difference is: {}", difference(&container));
}
