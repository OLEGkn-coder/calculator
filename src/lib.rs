fn check_bracket(input: &String) -> i32 {
    let mut res: i32 = 0;
    let mut new_input = input.clone();
    while new_input.contains('(') && new_input.contains(')') {
        let mut value_bracket: i32 = 0;
        let mut num_in_bracket: Vec<i32> = Vec::new();
        let mut sign_in_bracket: Vec<char> = Vec::new();
        let mut in_bracket: String = String::new();
        let input_chars: Vec<char> = new_input.chars().collect();

        let mut open_bracket: Option<usize> = None;
        let mut close_bracket: Option<usize> = None;

        for (a, &c) in input_chars.iter().enumerate() {
            if c == '(' {
                open_bracket = Some(a);
            } else if c == ')' && open_bracket.is_some() {
                close_bracket = Some(a);
                break;
            }
        }

        let index_start = open_bracket.unwrap();
        let index_end = close_bracket.unwrap();
        in_bracket = input_chars[index_start + 1..index_end].iter().collect();

        num_in_bracket = parse_number(&in_bracket);
        sign_in_bracket = parse_sign(&in_bracket);
        value_bracket = calc(num_in_bracket, sign_in_bracket);

        new_input.replace_range(index_start..=index_end, &value_bracket.to_string());
    }
    let number_input: Vec<i32> = parse_number(&new_input);
    let sign_input: Vec<char> = parse_sign(&new_input);

    res = calc(number_input, sign_input);
    print!("Result: {}", res);

    return res;
}

fn parse_number(input: &String) -> Vec<i32> {
    let mut number: Vec<i32> = input
        .replace("+", " ")
        .replace("-", " ")
        .replace("*", " ")
        .replace("/", " ")
        .split_whitespace()
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
        if a + 1 >= number.len() {
            break;
        }
        if ch[a] == '+' {
            res += number[a + 1];
        } else if ch[a] == '-' {
            res -= number[a + 1];
        }
    }

    return res;
}
