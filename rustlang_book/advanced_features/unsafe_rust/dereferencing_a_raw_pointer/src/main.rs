fn main() {
    let mut number = 5;

    let reference_1 = &number as *const i32;
    let reference_2 = &mut number as *mut i32;

    unsafe {
        println!("Reference 1 is: {}", *reference_1);
        println!("Reference 2 is: {}", *reference_2);
    }
}
