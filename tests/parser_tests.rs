use error::{CalcError, Res};
use Parser_calculator::check_bracket;
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
