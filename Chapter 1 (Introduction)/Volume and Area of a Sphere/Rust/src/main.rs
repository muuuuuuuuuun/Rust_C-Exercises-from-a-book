use std::io;

fn main() {
    // Declare a variable for radius input
    let mut input = String::new();

    // Input: asking for radius
    println!("Please enter a value for the radius:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input as a floating point number (f64)
    let r: f64 = input.trim().parse().expect("Invalid number");

    // Process: Calculate volume and surface area of the sphere
    let vol = (4.0 / 3.0) * std::f64::consts::PI * r.powi(3); // Volume of a sphere: (4/3) * π * r^3
    let area = 4.0 * std::f64::consts::PI * r.powi(2);        // Surface area of a sphere: 4 * π * r^2

    // Displaying output
    println!("The volume of the sphere is {:.2}.", vol); // {:.2} limits the number of decimal places
    println!("The surface area of the sphere is {:.2}.", area);
}