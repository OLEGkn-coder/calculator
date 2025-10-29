use core::num;
use std::io;

fn main() {
    println!("Welcome to the calculator");
    println!("y-y-a-a");
    println!("Enter a number to calculate: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let num = parse_into_a_number(&input);
    let sign = parse_sign(&input);
    let res = calc(num, sign);
    println!("Result: {:?}", res);
}

fn parse_into_a_number(input: &String) -> Vec<i32> {
    let mut number: Vec<i32> = input
        .replace("+", " ")
        .replace("-", " ")
        .replace("*", " ")
        .replace("/", " ")
        .split(" ")
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    println!("{:?}", number);
    return number;
}

fn parse_sign(input: &String) -> Vec<char> {
    let mut ch: Vec<char> = input
        .chars()
        .filter(|c| *c == '+' || *c == '-' || *c == '*' || *c == '/')
        .collect();

    println!("{:?}", ch);
    return ch;
}

fn calc(number: Vec<i32>, ch: Vec<char>) -> i32 {
    let mut res = number[0];
    for a in 0..number.len() - 1 {
        let mut num2 = number[a + 1];
        res = match ch[a] {
            '+' => res + num2,
            '-' => res - num2,
            '*' => res * num2,
            '/' => res / num2,
            _ => 0,
        }
    }
    return res;
}
