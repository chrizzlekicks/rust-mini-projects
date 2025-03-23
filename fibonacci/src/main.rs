use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!("Enter a number and get the Fibonacci number from it:");

        io::stdin().read_line(&mut input).expect("Cannot read line.");

        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input is not a valid number, try again.");
                continue;
            }
        };

        let position = fibonacci(input);

        println!("The Fibonacci number for {input} is {position}.");
        break;
    }
}

fn fibonacci(input: u64) -> u64 {
    if input == 0 {
        return 0
    };

    if input <= 2 {
        return 1
    }

    fibonacci(input - 1) + fibonacci(input - 2)
}
