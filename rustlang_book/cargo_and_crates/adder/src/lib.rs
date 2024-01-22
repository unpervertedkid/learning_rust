//! Adder
//! 
//! `adder` is a collection of utilities to make performing certain calculations more convenient


/// Adds two given numbers together
/// 
/// # Example
/// 
/// ```
/// assert_eq!(adder::add(10,10),20);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Adds one to the number given 
/// 
/// # Example
/// 
/// ```
/// assert_eq!(adder::add_one(4), 5);
pub fn add_one(number_to_add_one_to: usize) -> usize{
    number_to_add_one_to + 1
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_add_one() {
        assert_eq!(4, add_one(3));
    }
}
