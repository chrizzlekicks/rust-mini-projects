fn main() {
    let input = read_input("Enter the temperature you want to convert");

    let temperature = match input.trim().parse::<f64>() {
        Ok(temperature) => temperature,
        Err(_) => {
            println!("Temperature is invalid.");
            return;
        }
    };

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
    };
}

fn read_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}
