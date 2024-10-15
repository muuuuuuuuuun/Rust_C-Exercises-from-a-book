use std::io;

fn main() {
    // Declare variables for storing the two numbers
    let mut no1 = String::new();
    let mut no2 = String::new();

    // Prompt for the first number
    println!("Enter number 1:");
    io::stdin().read_line(&mut no1).expect("Failed to read input");
    
    // Prompt for the second number
    println!("Enter number 2:");
    io::stdin().read_line(&mut no2).expect("Failed to read input");

    // Convert the inputs from String to i32 (integer)
    let no1: i32 = no1.trim().parse().expect("Invalid number");
    let no2: i32 = no2.trim().parse().expect("Invalid number");

    // Perform the summation
    let sum = no1 + no2;

    // Output the result
    println!("The sum is {}.", sum);
    
}
