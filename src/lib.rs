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
    if input.contains("pow") {
        return to_power(input);
    }
    if input.contains("sqrt") {
        return to_sqrt(input);
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
    let replace_input: Vec<String> = input
        .replace("(", " ")
        .replace(")", " ")
        .split(" ")
        .map(|s| s.to_string())
        .collect();
    if replace_input.is_empty() {
        return Err(CalcError::EmptyExp);
    }
    let nums: String = replace_input[1].to_string();
    let numbers: Vec<i32> = nums
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
    let replcae_input: Vec<i32> = input
        .replace("(", " ")
        .replace(")", " ")
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .collect();
    if replcae_input.is_empty() {
        return Err(CalcError::EmptyExp);
    }
    let number = replcae_input[0];
    let mut res: i32 = 0;
    for i in 1..number {
        if i * i == number {
            res = i;
            break;
        }
    }
    Ok(res)
}

pub fn sin_func(input: &String) -> Res<f64> {
    let num = input.replace("sin( ", "").replace(")", "");
    let num: f64 = num.parse().unwrap();
    Ok(num.sin())
}

pub fn cos_func(input: &String) -> Res<f64> {
    let num = input.replace("cos( ", "").replace(")", "");
    let num: f64 = num.parse().unwrap();
    Ok(num.cos())
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
}
