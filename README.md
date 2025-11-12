# ParserCalculator, autor Oleh Moshenskyi

## Parsing a mathemtical expression

### В парсері ми будемо парсити математичний вираз на його основні складові: числа, знаки, дужки. Результат парсингу буде використаний для обрахування математичного виразу, враховуючи пріоритети: спочатку дужки потім множення/ділення і в кінці додавання та віднімання. Також калькулятор враховує вкладені дужки. Також калькулятор підносить число до степеня(pow(n, p)), обраховує квадратний корінь(sqrt(n)) та обчислює sin(n) та cos(n). Ось які функції реалізовано: 

- https://crates.io/crates/ParserCalculator

1. 
```text 
fn check_bracket(input: &String) -> i32{}
```
- Ця функція приймає наший вхідий рядок та перевіряє його на наявність дужок, якщо вони є то беремо вираз всередині них, обраховюємо та замінюємо вираз в дужках значенням яке вийшло після обчислень.

2. 
```text
fn parse_number(input: &String) -> Vec<i32> {}
```
- Ця функція використовується для парсингу математичного виразу, щоб видобути числа для подальшого обчислення. Для цього ми замінюємо всі знаки на пробіли і потім парсимо по пробілам наші числа.
- Приклад
```text
assert_eq!(parse_number(&"2+3*5-25".to_string()), vec![2, 3, 5, 25]);
```

3. 
```text
fn parse_sign(input: &String) -> Vec<char> {}
```
- Ця функція використовується для того, щоб видобути оператори для подальшого обчислення. Для цього ми просто використовуємо метод filtre щоб відфільтрувати всі знаки від інших символів та чисел.
- Приклад
```text
 assert_eq!(parse_number(&"2+3*5-25".to_string()), vec![+, *, -]);
```

4. 
```text
fn calc(mut number: Vec<i32>, ch: Vec<char>) -> i32 {}
```

- Основна функція обрахунку - після того як ми отримали наші числа та оператори ми обраховуємо загальне значення виразу й повератємо його. Спочатку ми шукаємо " * " та "/" щоб виконати ці дії, обчислюємо число яке стоїть на місці(a) індекса знака " * " або "/" з a+1 та отримуємо значення.
- Приклад
```text
 assert_eq!(parse_number(&"2+3*5-25".to_string()), -8);
```

5. 
```text
pub fn to_power(input: &String) -> Res<i32> {}
```

- Функція яка обраховує степінь числа. Беремо рядок pow(2,2) - парсимо його, дастаємо з виразу число та степінь до якого потрібно піднести.

```text
assert_eq!(check_bracket(&"pow(2,2)".to_string()).unwrap(), 4);
```

6. 
```text 
pub fn to_sqrt(input: &String) -> Res<i32> {}
```

- Функція яка знаходить квадратний корінь числа. за допомогою циклу поки не знайдемо пару чисел які при добутку дають задане число.

```text
assert_eq!(check_bracket(&"sqrt(16)".to_string()).unwrap(), 4);
```

7. 
```text
pub fn sin_func(input: &String) -> Res<i32> {}
```

- Функція що повертає значення заданого sin. За допомогою парсингу добуваємо це значення та передаємо його у вбудовану функцію sin()

```text
assert_eq!(check_bracket(&"sin(0)".to_string()).unwrap(), 0);
```

8. 
```text
pub fn cos_func(input: &String) -> Res<i32> {}
```

- Функція що повертає значення заданого cos. За допомогою парсингу добуваємо це значення та передаємо його у вбудовану функцію cos()

```text
assert_eq!(check_bracket(&"cos(0)".to_string()).unwrap(), 1);
```

9. 
```text
pub fn fac_func(input: &String) -> Res<i32> {}
```

- Функція що рахує факторіал, також підтримує подвійний факторіал та факторіальний вираз

```text
assert_eq!(check_bracket(&"(2+3)!".to_string()).unwrap(), 120);
```




10. 
```text
pub fn unary_func(input: &String) -> Res<i32> {} 
```

- Функція що рахує рахує унарний мінус. Також є можливість обрахування виразів з врахування "-"

```text
assert_eq!(check_bracket(&"-(2*(3+2))".to_string()).unwrap(), -10);
```


11. 
```text
pub fn abs_func(input: &String) -> Res<i32> {}
```

- Функція що рахує рахує модуль числа або виразу

```text
 assert_eq!(check_bracket(&"|2-5|".to_string()).unwrap(), 3);
```

12. 

```text
pub fn remove_spaces(input: &String) -> String {}
```

- За допомогою функції remove_spaces з виразу прибирається коментарі та пробіли 

```text
check_bracket(&"12 + 3 * 4 #comment\n+1".to_string()).unwrap(),
```

#### Також додані граматичні првила:
```text
/// Пропускає пробіли та переноси рядків.
WHITESPACE = _{ " " | "\n" }
/// Пропускає пробіли, таби, переноси і коментарі
whitespace_or_comment = _{ (" "| "\t" | "\n" | "#" ~ (!"\n" ~ ANY)*)* }
/// Ціле число з необов'язковим мінусом
number = @{ "-"? ~ ASCII_DIGIT+ ~ whitespace_or_comment}
/// Арифметичний вираз із пріоритетами операцій
expression = {priority ~ (second_priority ~ priority)* ~ whitespace_or_comment}
/// Група термів високого пріоритету("*", "/")
priority = { (unary | in_brackets) ~ (first_priority ~(unary | in_brackets))* ~ whitespace_or_comment}
/// Вираз у дужках, функцій, або модуль
in_brackets = {function| abs | number | "(" ~ expression ~ ")" ~ whitespace_or_comment}
/// Оператори першого пріоритету("*", "/")
first_priority = { "*" | "/" ~ whitespace_or_comment}
/// Оператори другого пріоритету("+", "-")
second_priority = { "+" | "-" ~ whitespace_or_comment}
/// Виклик функцій
function = {function_name ~ "(" ~ function_arg ~ ")" ~ whitespace_or_comment }
/// Назва функцій, що підтримуються
function_name = { "pow" | "sqrt" | "sin" | "cos"}
/// Аргументи функцій, розділені комами
function_arg = { expression ~ ("," ~ expression)* ~ whitespace_or_comment }
/// Повний вираз від початку до кінця вхідного рядка
calculation = { SOI ~ expression ~ EOI }
/// Факторіал для числа або виразу у дужках
factorial = { in_brackets ~ ("!")+ ~ whitespace_or_comment } 
/// Унарний мінус перед виразом
unary = { "-" ~ in_brackets ~ whitespace_or_comment }
/// Абсолютне значення виразу(модуль)
abs = { "|" ~ expression ~ "|" ~ whitespace_or_comment }
```
