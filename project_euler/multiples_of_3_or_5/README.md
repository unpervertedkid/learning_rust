# Multiples of 3 or 5

If we list the all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.


## Approach

### Initial thoughts

We can create two iterators, one that yields multiples of 3 and another one that yields multiples of 5.

We can then call next() on both iterators so long as the condition is met that the next value is less than 1000 and add the values together.

### Simple but efficient solution
Create one iterator that generates multiples of 3 and 5 and add them together.
Call the sum() function on the iterator and pass in the range of numbers to generate.

