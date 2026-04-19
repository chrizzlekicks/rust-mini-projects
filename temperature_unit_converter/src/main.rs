use degree_converter::{Temperature, Unit};

fn main() {
    let degree = read_degree();
    let source_unit =
        read_unit("Which unit is the temperature currently in? (Fahrenheit or Celsius):");

    let initial = Temperature::new(degree, source_unit);

    let target_unit = read_unit("Into which unit you would like to convert to:");
    let converted = initial.to(target_unit);

    println!(
        "{} degrees {:?} is {:.2} degrees {:?}.",
        initial.degree(),
        initial.unit(),
        converted.degree(),
        converted.unit(),
    );
}

fn read_degree() -> f64 {
    loop {
        let degree_input = read_input("Enter the temperature you want to convert:");
        match degree_input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Temperature is invalid. Please try again.");
            }
        }
    }
}

fn read_unit(prompt: &str) -> Unit {
    loop {
        let unit_input = read_input(prompt);
        match unit_input.trim().to_lowercase().as_str() {
            "fahrenheit" | "f" => return Unit::Fahrenheit,
            "celsius" | "c" => return Unit::Celsius,
            _ => {
                println!("Invalid input for the specified unit!");
            }
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
