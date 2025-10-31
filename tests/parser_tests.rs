use Parser_calculator::*;

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
