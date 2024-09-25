// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
*/

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    
    /* Complete the code after this line */ 
    let squares = getVector(n);

    let sum_of_squares= sumOfSquares(squares.clone());
    let square_of_sum= squareOfSums(squares);
    
    let difference = square_of_sum - sum_of_squares;

    println!("The difference is: {}", difference);

}

fn getVector(num1: i32) -> Vec<i32>{
    let mut squares = Vec::new();
    for i in 1..num1{
        squares.push(i);
    }
    squares
    
}



fn sumOfSquares(squares: Vec<i32>) -> i32{
    // Sums Calculation
    let mut sum = 0;
    for &num in &squares {
        sum += num * num;
    }
    sum
}

fn squareOfSums(squares: Vec<i32>) -> i32{
    // Sums Calculation
    let sum: i32 = squares.iter().sum();
    sum * sum
}
