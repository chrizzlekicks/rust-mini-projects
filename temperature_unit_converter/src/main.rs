fn main() {
    println!("Enter the temperature in degree you want to convert:");

    let mut temperature = String::new();

    std::io::stdin().read_line(&mut temperature).expect("Failed to read line.");

    let temperature: f64 = temperature.trim().parse().expect("Temperature is invalid."); 

    println!("To which unit (Fahrenheit or Celsius) you would like to convert:");

    let mut unit = String::new();

    std::io::stdin().read_line(&mut unit).expect("Failed to read line.");

    match unit.trim().to_lowercase().as_str() {
        "fahrenheit" => {
            let converted = celsius_to_fahrenheit(temperature);
            println!("{temperature} degree Celsius is {converted} degree Fahrenheit.");
        },
        "celsius" => {
            let converted = fahrenheit_to_celsius(temperature);
            println!("{temperature} degree Fahrenheit is {converted} degree Celsius.");
        },
        _ => println!("Invalid input for the specified unit!"),
    };
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0 
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}
