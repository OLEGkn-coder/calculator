use std::io;

fn main() {
    println!("Welcome to the calculator");
    println!("y-y-a-a");
    println!("Enter a number to calculate: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    check_bracket(&input);
}

fn check_bracket(input: &String) -> i32 {
    let mut is_bracket = false;
    let mut res: i32 = 0;

    if input.contains("(") && input.contains(")") {
        is_bracket = true;
    } else {
        is_bracket = false;
    }

    if is_bracket {
        let mut value_bracket: i32 = 0;
        let mut num_in_bracket: Vec<i32> = Vec::new();
        let mut sign_in_bracket: Vec<char> = Vec::new();
        let mut in_bracket: String = String::new();
        let mut bracket_position: Vec<usize> = Vec::new();
        let input_chars: Vec<char> = input.chars().collect();
        let mut c = false;

        for a in 0..input_chars.len() {
            if input_chars[a] == '(' {
                c = true;
                bracket_position.push(a);
                continue;
            } else if input_chars[a] == ')'
                && input_chars[a + 1] != '+'
                && input_chars[a + 1] != '-'
                && input_chars[a + 1] != '*'
                && input_chars[a + 1] != '/'
            {
                c = false;
                bracket_position.push(a);
                break;
            } else if input_chars[a] == ')'
                || input_chars[a + 1] == '+'
                || input_chars[a + 1] == '-'
                || input_chars[a + 1] == '*'
                || input_chars[a + 1] == '/'
            {
                c = true;
                continue;
            }
            if c {
                in_bracket.push(input_chars[a]);
            }
        }
        num_in_bracket = parse_number(&in_bracket);
        sign_in_bracket = parse_sign(&in_bracket);
        value_bracket = calc(num_in_bracket, sign_in_bracket);

        let index_start = bracket_position[0];
        let index_end = bracket_position[bracket_position.len() - 1];
        let mut new_input = input.clone();

        new_input.replace_range(index_start..=index_end, &value_bracket.to_string());

        let number_input: Vec<i32> = parse_number(&new_input);
        let sign_input: Vec<char> = parse_sign(&new_input);

        res = calc(number_input, sign_input);
    } else {
        let number_input: Vec<i32> = parse_number(input);
        let number_sign: Vec<char> = parse_sign(input);
        res = calc(number_input, number_sign);
    }
    return res;
}

fn parse_number(input: &String) -> Vec<i32> {
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

fn calc(mut number: Vec<i32>, ch: Vec<char>) -> i32 {
    if number.is_empty() {
        return 0;
    }

    for b in 0..ch.len() {
        if b + 1 >= number.len() {
            break;
        }
        if ch[b] == '*' {
            number[b] = number[b] * number[b + 1];
            number[b + 1] = 0;
        }
        if ch[b] == '/' {
            number[b] = number[b] / number[b + 1];
            number[b + 1] = 0;
        }
    }

    let mut res = number[0];
    for a in 0..ch.len() {
        if ch[a] == '+' {
            res += number[a + 1];
        } else if ch[a] == '-' {
            res -= number[a + 1];
        }
    }

    return res;
}
