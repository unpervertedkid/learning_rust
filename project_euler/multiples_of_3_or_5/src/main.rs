struct MultiplesOf {
    current_multiple: i32,
    multiple_of: i32,
}

impl MultiplesOf {
    fn new(multiple_of: i32) -> Self {
        MultiplesOf {
            current_multiple: multiple_of,
            multiple_of,
        }
    }
}

impl Iterator for MultiplesOf {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_multiple;

        self.current_multiple += self.multiple_of;

        Some(current)
    }
}
fn main() {
    // Naive trial
    let multiples_of_3_below_1000 = MultiplesOf::new(3).take_while(|multiple| multiple < &1000);
    let multiples_of_5_below_1000 = MultiplesOf::new(5).take_while(|multiple| multiple < &1000);

    let multiples_of_3_sum: i32 = multiples_of_3_below_1000.sum();
    let multiples_of_5_sum: i32 = multiples_of_5_below_1000.sum();

    // However, this is wrong as some numbers which are both multiples of 3 and 5 are summed twice
    let _total_sum = multiples_of_3_sum + multiples_of_5_sum;

    // Simple yet correct implementation
    let multiples_of_3_or_5_below_1000 =
        (1..1000).filter(|number| number % 3 == 0 || number % 5 == 0);

    let total_sum: i32 = multiples_of_3_or_5_below_1000.sum();
    println!("The sum of all multiples of 3 and 5 below 1000 is: {total_sum}");
}
