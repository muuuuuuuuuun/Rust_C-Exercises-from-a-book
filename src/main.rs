use std::io::stdin;

fn main() {
    println!("What do you want to convert. Please select!");

    println!("[F] for converting from Celsius to Fahrenheit!");
    println!("[C] for converting from Fahrenheit to Celsius!");

    let mut choice: String = String::new();

    // Correct syntax for read_line and expect
    stdin().read_line(&mut choice).expect("Please provide your choice.");

    let choice: &str = &choice.trim().to_uppercase();

    if choice != "F" && choice != "C" {
        println!("Not a valid choice. Exiting");
        return;
    }

    println!("Please provide temperature.");
    let mut temperature: String = String::new();

    stdin().read_line(&mut temperature).unwrap_or_default();

    // Parse temperature to f64
    let temperature: f64 = temperature.trim().parse().unwrap_or_default();

    // Correct use of as_str and casting
    let new_tempe: f64 = match choice {
        "F" => (temperature * (9f64 / 5f64)) + 32f64,
        "C" => (temperature - 32f64) * (5f64 / 9f64),
        _ => 0.00,
    };

    // Output the result
    println!("The converted temperature is {} {}", new_tempe, if choice == "F" { "Fahrenheit" } else { "Celsius" });
}