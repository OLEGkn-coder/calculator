#![doc = include_str!("../README.md")]

pub mod error;
use std::char;

use error::{CalcError, Res};
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ParserCalculator;

/// Це парсер математичних виразів який бере наш вираз(input) та парсить його на
/// основні складові: числа, знаки, дужки.
/// Вираз складається з:
/// - Числа
/// - Нуль або більше операторів та чисел
/// - Нуль або більше дужок з виразами всередині
///
/// Порядок обчислення стандартний:
/// 1. Дужки
/// 2. Множення і ділення
/// 3. Додавання і віднімання

pub fn pars(input: &str) -> Res<i32> {
    if input.trim().is_empty() {
        return Err(CalcError::EmptyExp);
    }
    let input = input.trim();
    check_bracket(&input.to_string())
}
/// Функція що видаляє коментарі та зайві пробіли
pub fn remove_spaces(input: &String) -> String {
    let mut res = String::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '#' => {
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '\n' {
                        break;
                    }
                }
            }
            _ => {
                res.push(c);
                chars.next();
            }
        }
    }
    res
}
/// Функція шукає дужки у нашому виразі, якщо знаходить то
/// обчислюємо значення в дужках та замінюємо цей вираз з дужками на саме значення.
pub fn check_bracket(input: &String) -> Res<i32> {
    let input = remove_spaces(input);
    if input.contains("pow") {
        return to_power(&input);
    }
    if input.contains("sqrt") {
        return to_sqrt(&input);
    }
    if input.contains("sin") {
        return sin_func(&input);
    }
    if input.contains("cos") {
        return cos_func(&input);
    }
    if input.contains("!") && input.contains('(') && input.contains(')') {
        return fac_func(&input);
    }
    if input.starts_with("-") {
        return unary_func(&input);
    }
    if input.starts_with("|") && input.ends_with("|") {
        return abs_func(&input);
    }
    let mut new_input = input.clone();
    while new_input.contains('(') && new_input.contains(')') {
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
        if open_bracket.is_none() || close_bracket.is_none() {
            return Err(CalcError::NoBracket);
        }

        let index_start = open_bracket.unwrap();
        let index_end = close_bracket.unwrap();
        let in_bracket = input_chars[index_start + 1..index_end].iter().collect();

        let num_in_bracket = parse_number(&in_bracket);
        let sign_in_bracket = parse_sign(&in_bracket);
        let value_bracket = calc(num_in_bracket, sign_in_bracket)?;

        new_input.replace_range(index_start..=index_end, &value_bracket.to_string());
    }
    let number_input: Vec<i32> = parse_number(&new_input);
    let sign_input: Vec<char> = parse_sign(&new_input);

    calc(number_input, sign_input)
}

/// Парсимо input на числа.
/// Для цього ми замінюємо всі знаки на пробіли і потім
/// парсимо по пробілам наші числв.
/// Число - це одна або більше цифр(0-9).
pub fn parse_number(input: &String) -> Vec<i32> {
    let number: Vec<i32> = input
        .replace("+", " ")
        .replace("-", " ")
        .replace("*", " ")
        .replace("/", " ")
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();
    println!("{:?}", number);
    number
}

/// Парсимо input на знаки.
/// Для цього ми просто використовуємо метод filtre щоб відфільтрувати
/// всі знаки від інших символів та чисел.
/// Підтримуються знаки: +, -. *, /

pub fn parse_sign(input: &String) -> Vec<char> {
    let ch: Vec<char> = input
        .chars()
        .filter(|c| *c == '+' || *c == '-' || *c == '*' || *c == '/')
        .collect();

    println!("{:?}", ch);
    ch
}

/// Функція яка обчислює значення виразу приймаючи як аргумент ветор
/// з числами та знаками. Спочатку ми шукаємо "*" та "/" щоб виконати ці дії,
/// обчислюємо число яке стоїть на місці(a) індекса знака "*" або "/" з a+1
/// та отримуємо значення. Потім додавання та віднімання.

pub fn calc(mut number: Vec<i32>, mut ch: Vec<char>) -> Res<i32> {
    if number.is_empty() {
        return Err(CalcError::EmptyExp);
    }
    let mut b = 0;
    while b < ch.len() {
        if b + 1 >= number.len() {
            break;
        }
        if ch[b] == '*' {
            number[b] *= number[b + 1];
            number.remove(b + 1);
            ch.remove(b);
        } else if ch[b] == '/' {
            if number[b + 1] == 0 {
                return Err(CalcError::DivByZero);
            }
            number[b] /= number[b + 1];
            number.remove(b + 1);
            ch.remove(b);
        } else {
            b += 1;
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

    Ok(res)
}

pub fn to_power(input: &String) -> Res<i32> {
    let mut res: i32 = 1;
    let replace_input: String = input.replace("pow(", "").replace(")", "");
    if replace_input.is_empty() {
        return Err(CalcError::EmptyExp);
    }
    let numbers: Vec<i32> = replace_input
        .replace(",", " ")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    let number = numbers[0];
    let power = numbers[1];
    for _ in 0..power as usize {
        res *= number;
    }
    Ok(res)
}

pub fn to_sqrt(input: &String) -> Res<i32> {
    let replcae_input: String = input.replace("sqrt(", "").replace(")", "");

    if replcae_input.is_empty() {
        return Err(CalcError::EmptyExp);
    }
    let number = replcae_input.parse().unwrap();
    let mut res: i32 = 0;
    for i in 1..number {
        if i * i == number {
            res = i;
            break;
        }
    }
    Ok(res)
}

pub fn sin_func(input: &String) -> Res<i32> {
    let num = input.replace("sin(", "").replace(")", "");
    let num: f64 = num.parse().unwrap();
    Ok(num.to_radians().sin().round() as i32)
}

pub fn cos_func(input: &String) -> Res<i32> {
    let num = input.replace("cos(", "").replace(")", "");
    let num: f64 = num.parse().unwrap();
    Ok(num.to_radians().cos().round() as i32)
}

pub fn fac_func(input: &String) -> Res<i32> {
    if input.is_empty() {
        return Err(CalcError::EmptyExp);
    }
    let mut res = 1;
    let mut count = 0;
    for i in input.chars() {
        if i == '!' {
            count += 1;
        }
    }
    if count == 1 {
        let num = input.replace("!", "");
        let num: i32 = if num.contains('(') || num.contains('+') || num.contains('-') {
            check_bracket(&num)?
        } else {
            num.parse().map_err(|_| CalcError::InvalidInput)?
        };
        for i in 1..=num {
            res *= i;
        }
    } else if count == 2 {
        let num = input.replace("(", "").replace("!", "").replace(")", "");
        let num: i32 = if num.contains('(') || num.contains('+') || num.contains('-') {
            check_bracket(&num)?
        } else {
            num.parse().map_err(|_| CalcError::InvalidInput)?
        };

        for i in 1..=num {
            res *= i;
        }
        let temp = res;
        res = 1;
        for i in 1..=temp {
            res *= i;
        }
    } else {
        return Err(CalcError::InvalidInput);
    }

    Ok(res)
}

pub fn unary_func(input: &String) -> Res<i32> {
    let num = input.trim_matches('-').to_string();
    let num: i32 = if num.contains('(')
        || num.contains('+')
        || num.contains('-')
        || num.contains('*')
        || num.contains('/')
    {
        check_bracket(&num)?
    } else {
        num.parse().map_err(|_| CalcError::InvalidInput)?
    };
    Ok(-num)
}

pub fn abs_func(input: &String) -> Res<i32> {
    let num = input.trim_matches('|').to_string();
    let num: i32 = if num.contains('(')
        || num.contains('+')
        || num.contains('-')
        || num.contains('*')
        || num.contains('/')
    {
        check_bracket(&num)?
    } else {
        num.parse().map_err(|_| CalcError::InvalidInput)?
    };
    Ok(num.abs())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_one() {
        assert_eq!(check_bracket(&"12+(3*1)*2".to_string()).unwrap(), 18);
    }

    #[test]
    fn test_number_two() {
        assert_eq!(check_bracket(&"(2+3)*5-25".to_string()).unwrap(), 0);
    }

    #[test]

    fn test_number_three() {
        assert_eq!(check_bracket(&"2*(2*(2*2))".to_string()).unwrap(), 16);
    }

    #[test]

    fn test_number_four() {
        assert_eq!(
            check_bracket(&"200+200+100*(100+100)-(100-90)".to_string()).unwrap(),
            20390
        );
    }

    #[test]

    fn test_number_five() {
        assert_eq!(
            check_bracket(
                &"123+387-321*(322-2)/313+1000-23*23-(3211)/3+3000-1*423+(3*(3*3))-1532/2+4*(231)"
                    .to_string()
            )
            .unwrap(),
            2345
        );
    }
    #[test]

    fn test_power_one() {
        assert_eq!(check_bracket(&"pow(2,2)".to_string()).unwrap(), 4);
    }

    #[test]

    fn test_power_two() {
        assert_eq!(check_bracket(&"pow(9,5)".to_string()).unwrap(), 59049);
    }

    #[test]

    fn test_sqrt_one() {
        assert_eq!(check_bracket(&"sqrt(16)".to_string()).unwrap(), 4);
    }

    #[test]

    fn test_sqrt_two() {
        assert_eq!(check_bracket(&"sqrt(100)".to_string()).unwrap(), 10);
    }
    #[test]

    fn test_sin_one() {
        assert_eq!(check_bracket(&"sin(0)".to_string()).unwrap(), 0);
    }

    #[test]

    fn test_sin_two() {
        assert_eq!(check_bracket(&"sin(90)".to_string()).unwrap(), 1);
    }

    #[test]

    fn test_cos_one() {
        assert_eq!(check_bracket(&"cos(0)".to_string()).unwrap(), 1);
    }

    #[test]

    fn test_cos_two() {
        assert_eq!(check_bracket(&"cos(90)".to_string()).unwrap(), 0);
    }

    #[test]

    fn test_fac_one() {
        assert_eq!(check_bracket(&"(5)!".to_string()).unwrap(), 120);
    }

    #[test]

    fn test_fac_two() {
        assert_eq!(check_bracket(&"(2+1)!!".to_string()).unwrap(), 720);
    }

    #[test]

    fn test_fac_three() {
        assert_eq!(check_bracket(&"(2+3)!".to_string()).unwrap(), 120);
    }

    #[test]

    fn test_fac_four() {
        assert_eq!(check_bracket(&"(3)!!".to_string()).unwrap(), 720);
    }
    #[test]

    fn test_unary_one() {
        assert_eq!(check_bracket(&"-5".to_string()).unwrap(), -5);
    }

    #[test]

    fn test_unary_two() {
        assert_eq!(check_bracket(&"-(3+2)".to_string()).unwrap(), -5);
    }

    #[test]

    fn test_unary_three() {
        assert_eq!(check_bracket(&"-(2*(3+2))".to_string()).unwrap(), -10);
    }

    #[test]

    fn test_abs_one() {
        assert_eq!(check_bracket(&"|-5|".to_string()).unwrap(), 5);
    }

    #[test]

    fn test_abs_two() {
        assert_eq!(check_bracket(&"|2-5|".to_string()).unwrap(), 3);
    }

    #[test]

    fn test_remove_spaces_one() {
        assert_eq!(check_bracket(&"12+3*4 #вираз".to_string()).unwrap(), 24);
    }

    #[test]

    fn test_remove_spaces_two() {
        assert_eq!(
            check_bracket(&"12 + 3 * 4 #вираз\n+1".to_string()).unwrap(),
            25
        );
    }
}
