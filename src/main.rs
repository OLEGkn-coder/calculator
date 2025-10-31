use std::io;

fn main() {
    println!("Welcome to the calculator");
    println!("y-y-a-a");
    println!("Enter a number to calculate: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let res = Parser_calculator::check_bracket(&input.trim().to_string());
    println!("Result: {}", res.unwrap());
}
