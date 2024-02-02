# Largest Prime Factor

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?

## Solution

### Initial thoughts

My initial thoughts are:

1. Create an iterator that yields prime numbers.
2. For each prime number, divide the target number by the prime number.
3. If the remainder is a whole number, the prime number is a factor of the target number.
4. If prime number is a factor, Minus the result of the division of the target number by the prime number from the target number.
5. Repeat steps 2-4 until the target number is 1.
6. The last prime number used to divide the target number is the largest prime factor of the target number.
7. Return the largest prime factor.

### Optimizations

I can optimize the solution by:

1. Only checking prime numbers up to the square root of the target number.
2. If the target number is a prime number, return the target number as the largest prime factor.
