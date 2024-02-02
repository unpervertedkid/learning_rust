fn main() {
    println!("Largest prime factor of 13195 is: {}", largest_prime_factor(13195) );
    println!("Largest prime factor of  600851475143 is: {}", largest_prime_factor(600851475143) );
}

fn largest_prime_factor(n: u64) -> u64 {
    let mut number = n;
    let mut factor = 2;
    
    // Loop until the square of the factor is greater than the number
    while factor * factor <= number {
        if number % factor == 0 {
            number /= factor;
        } else {
            factor += 1;
        }
    }
    number
}
