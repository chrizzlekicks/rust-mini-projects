use degree_converter::{celsius_to_fahrenheit, fahrenheit_to_celsius};

fn main() {
    let temperature = read_temperature();

    let input = read_input("Which unit would you like to convert to? (Fahrenheit or Celsius):");
    let unit = input.trim().to_lowercase();

    match unit.as_str() {
        "fahrenheit" | "f" => {
            let converted = celsius_to_fahrenheit(temperature);
            println!(
                "{temperature} degrees Celsius is {:.2} degrees Fahrenheit.",
                converted
            );
        }
        "celsius" | "c" => {
            let converted = fahrenheit_to_celsius(temperature);
            println!(
                "{temperature} degrees Fahrenheit is {:.2} degrees Celsius.",
                converted
            );
        }
        _ => println!("Invalid input for the specified unit!"),
    }
}

fn read_temperature() -> f64 {
    loop {
        let input = read_input("Enter the temperature you want to convert:");
        match input.trim().parse::<f64>() {
            Ok(temperature) => {
                return temperature;
            }
            Err(_) => println!("Temperature is invalid. Please try again."),
        }
    }
}

fn read_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input
}
