static mut COUNTER: u32 = 0;

fn add_to_counter(increment: u32) {
    unsafe {
        COUNTER += increment;
    }
}
fn main() {
    add_to_counter(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
