use pest::Parser;
use Parser_calculator::{ParserCalculator, Rule};
// number
#[test]
fn test_rule_number_pos() {
    let res = ParserCalculator::parse(Rule::number, "89");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "89");
}

#[test]
fn test_rule_number_neg() {
    let res = ParserCalculator::parse(Rule::number, "-5");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "-5");
}

#[test]
fn test_rule_number_inv() {
    let res = ParserCalculator::parse(Rule::number, "abs");
    assert!(res.is_err());
}

//first_priority

#[test]
fn test_rule_first_priority_mult() {
    let res = ParserCalculator::parse(Rule::first_priority, "*");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "*");
}

#[test]
fn test_rule_first_priority_dev() {
    let res = ParserCalculator::parse(Rule::first_priority, "/");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "/");
}

fn test_rule_first_priority_inv() {
    let res = ParserCalculator::parse(Rule::first_priority, "+");
    assert!(res.is_err());
}

//second_priority

#[test]
fn test_rule_second_priority_add() {
    let res = ParserCalculator::parse(Rule::second_priority, "+");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "+");
}

#[test]
fn test_rule_second_priority_sub() {
    let res = ParserCalculator::parse(Rule::second_priority, "-");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "-");
}

#[test]
fn test_rule_second_priority_inv() {
    let res = ParserCalculator::parse(Rule::second_priority, "*");
    assert!(res.is_err());
}

//function_name

#[test]
fn test_rule_function_name_pow() {
    let res = ParserCalculator::parse(Rule::function_name, "pow");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "pow");
}
#[test]
fn test_rule_function_name_sqrt() {
    let res = ParserCalculator::parse(Rule::function_name, "sqrt");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "sqrt");
}
#[test]
fn test_rule_function_name_sin() {
    let res = ParserCalculator::parse(Rule::function_name, "sin");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "sin");
}
#[test]
fn test_rule_function_name_cos() {
    let res = ParserCalculator::parse(Rule::function_name, "cos");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "cos");
}
#[test]
fn test_rule_function_name_inv() {
    let res = ParserCalculator::parse(Rule::function_name, "abs");
    assert!(res.is_err());
}
//function_arg
#[test]
fn test_rule_function_arg_one() {
    let res = ParserCalculator::parse(Rule::function_arg, "2");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "2");
}
#[test]
fn test_rule_function_arg_two() {
    let res = ParserCalculator::parse(Rule::function_arg, "2, 5");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "2, 5");
}
//function
#[test]
fn test_rule_function_pow() {
    let res = ParserCalculator::parse(Rule::function, "pow(2,2)");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "pow(2,2)");
}
#[test]
fn test_rule_function_sqrt() {
    let res = ParserCalculator::parse(Rule::function, "sqrt(2)");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "sqrt(2)");
}
#[test]
fn test_rule_function_sin() {
    let res = ParserCalculator::parse(Rule::function, "sin(90)");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "sin(90)");
}
#[test]
fn test_rule_function_cos() {
    let res = ParserCalculator::parse(Rule::function, "cos(0)");
    assert!(res.is_ok());
    let is_ok = res.unwrap().next().unwrap().as_str();
    assert_eq!(is_ok, "cos(0)");
}

//in_brackets
#[test]
fn test_rule_in_brackets_num() {
    let res = ParserCalculator::parse(Rule::in_brackets, "81");
    assert!(res.is_ok());
}
#[test]
fn test_rule_in_brackets_exp() {
    let res = ParserCalculator::parse(Rule::in_brackets, "2+2*3");
    assert!(res.is_ok());
}
#[test]
fn test_rule_in_brackets_long_exp() {
    let res = ParserCalculator::parse(Rule::in_brackets, "2+(2*7)");
    assert!(res.is_ok());
}
#[test]
fn test_rule_in_brackets_fun_name() {
    let res = ParserCalculator::parse(Rule::in_brackets, "sin(90)");
    assert!(res.is_ok());
}

//priority
#[test]
fn test_rule_priority_num() {
    let res = ParserCalculator::parse(Rule::priority, "2");
    assert!(res.is_ok());
}

#[test]
fn test_rule_priority_mult() {
    let res = ParserCalculator::parse(Rule::priority, "2 * 3");
    assert!(res.is_ok());
}

#[test]
fn test_rule_priority_div() {
    let res = ParserCalculator::parse(Rule::priority, "4/2");
    assert!(res.is_ok());
}

#[test]
fn test_rule_priority_exp() {
    let res = ParserCalculator::parse(Rule::priority, "(2*2) / 2 + 3");
    assert!(res.is_ok());
}

//expressions
#[test]
fn test_rule_exp_add() {
    let res = ParserCalculator::parse(Rule::expression, "2 + 2");
    assert!(res.is_ok());
}
#[test]
fn test_rule_exp_sub() {
    let res = ParserCalculator::parse(Rule::expression, "2 - 2");
    assert!(res.is_ok());
}

#[test]
fn test_rule_exp_mul() {
    let res = ParserCalculator::parse(Rule::expression, "2 * 2");
    assert!(res.is_ok());
}

#[test]
fn test_rule_exp_div() {
    let res = ParserCalculator::parse(Rule::expression, "2 / 2");
    assert!(res.is_ok());
}

#[test]
fn test_rule_exp_dif() {
    let res = ParserCalculator::parse(Rule::expression, "8*(2+2)-(3*(4-3))+1-10*1000");
    assert!(res.is_ok());
}
//calculation

#[test]
fn test_rule_cal_mul() {
    let res = ParserCalculator::parse(Rule::calculation, "2 * 2");
    assert!(res.is_ok());
}

#[test]
fn test_rule_cal_div() {
    let res = ParserCalculator::parse(Rule::calculation, "2 / 2");
    assert!(res.is_ok());
}

#[test]
fn test_rule_cal_dif() {
    let res = ParserCalculator::parse(Rule::calculation, "8*(2+2)-(3*(4-3))+1-10*1000");
    assert!(res.is_ok());
}

#[test]
fn test_rule_cal_add() {
    let res = ParserCalculator::parse(Rule::calculation, "2 + 2");
    assert!(res.is_ok());
}

#[test]
fn test_rule_cal_sub() {
    let res = ParserCalculator::parse(Rule::calculation, "2 - 2");
    assert!(res.is_ok());
}
/// factorial

#[test]
fn test_rule_factorial() {
    let res = ParserCalculator::parse(Rule::factorial, "5!");
    assert!(res.is_ok());
}

#[test]
fn test_rule_factorial_zero() {
    let res = ParserCalculator::parse(Rule::factorial, "0!");
    assert!(res.is_ok());
}

#[test]
fn test_rule_factorial_one() {
    let res = ParserCalculator::parse(Rule::factorial, "1!");
    assert!(res.is_ok());
}

#[test]
fn test_rule_factorial_exp() {
    let res = ParserCalculator::parse(Rule::factorial, "(32+2)!");
    assert!(res.is_ok());
}

#[test]
fn test_rule_factorial_double() {
    let res = ParserCalculator::parse(Rule::factorial, "(5-1)!!");
    assert!(res.is_ok());
}

#[test]
fn test_rule_factorial_inv() {
    let res = ParserCalculator::parse(Rule::factorial, "5");
    assert!(res.is_err());
}
