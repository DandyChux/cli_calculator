use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the calculation (e.g. 1 + 2): ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Please enter a valid operation.");
        return;
    }

    let num1: f64 = tokens[0].parse().expect("Invalid number");
    let operator = tokens[1];
    let num2: f64 = tokens[2].parse().expect("Invalid number");

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Result: {}", result);
}
