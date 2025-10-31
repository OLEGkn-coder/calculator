#![doc = include_str!("../README.md")]
pub mod error;
use error::{CalcError, Res};

/// Це парсер математичних виразів який бере наш вираз(input) та парсить його на
/// основні складові: числа, знаки, дужки.
/// # Граматика:
///
/// Grammar Rule: expresion
///
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
/// Grammar Rule: expresion(дужки)
///
/// Функція шукає дужки у нашому виразі, якщо знаходить то
/// обчислюємо значення в дужках та замінюємо цей вираз з дужками на саме значення.
pub fn check_bracket(input: &String) -> Res<i32> {
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

/// Grammar Rule: number
///
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

/// Grammar Rule: sign
///
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

pub fn calc(mut number: Vec<i32>, ch: Vec<char>) -> Res<i32> {
    if number.is_empty() {
        return Err(CalcError::EmptyExp);
    }

    for b in 0..ch.len() {
        if b + 1 >= number.len() {
            break;
        }
        if ch[b] == '*' {
            number[b] *= number[b + 1];
            number[b + 1] = 0;
        }
        if ch[b] == '/' {
            if number[b + 1] == 0 {
                return Err(CalcError::DivByZero);
            }
            number[b] /= number[b + 1];
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

    Ok(res)
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
}
