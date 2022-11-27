use std::env::{args, Args}; // Import module for CLI arguments

fn main() {
    let mut args: Args = args();

    // First argument is the location of the compiled binary (index=0), we skip it
    let first: String = args.nth(1).unwrap();
    // When the 2nd argument is accessed, iterator's next element becomes the first
    // Convert operator to char first by calling chars method
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    // Call operate
    let result = operate(operator, first_number, second_number);
    // Format results to String
    println!("{}", output(first_number, operator, second_number, result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator used."),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{first_number} {operator} {second_number} = {result}")
}
