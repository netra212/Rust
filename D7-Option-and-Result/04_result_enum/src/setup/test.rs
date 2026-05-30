#[test]
fn test_result_construction_used() {
    let source_code = get_source_code();
    assert!(source_code.contains("MyResult::Ok"), "Should construct MyResult::Ok");
    assert!(source_code.contains("MyResult::Err"), "Should construct MyResult::Err");
    assert!(source_code.contains("match ") || source_code.contains("if "), "Should use pattern matching or conditional logic");
}

#[test]
fn test_create_success_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{create_success, MyResult};
        let result = create_success(42);
        assert_eq!(result, MyResult::Ok(42));
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::{create_success, MyResult};
        let result = create_success(42);
        assert_eq!(result, MyResult::Ok(42), "create_success should create Ok variant");
    }
}

#[test]
fn test_validate_age_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{validate_age, MyResult};
        let result = validate_age(25);
        assert_eq!(result, MyResult::Ok(25));
        let result2 = validate_age(-5);
        assert!(matches!(result2, MyResult::Err(_)));
        let result3 = validate_age(150);
        assert!(matches!(result3, MyResult::Err(_)));
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::{validate_age, MyResult};
        let result = validate_age(25);
        assert_eq!(result, MyResult::Ok(25), "validate_age should accept valid ages");
        let result2 = validate_age(-5);
        assert!(matches!(result2, MyResult::Err(_)), "validate_age should reject negative ages");
        let result3 = validate_age(150);
        assert!(matches!(result3, MyResult::Err(_)), "validate_age should reject ages over 120");
    }
}

#[test]
fn test_parse_number_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{parse_number, MyResult};
        let result = parse_number("123");
        assert_eq!(result, MyResult::Ok(123));
        let result2 = parse_number("abc");
        assert!(matches!(result2, MyResult::Err(_)));
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::{parse_number, MyResult};
        let result = parse_number("123");
        assert_eq!(result, MyResult::Ok(123), "parse_number should parse valid numbers");
        let result2 = parse_number("abc");
        assert!(matches!(result2, MyResult::Err(_)), "parse_number should return error for invalid input");
    }
}

#[test]
fn test_divide_numbers_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{divide_numbers, MyResult};
        let result = divide_numbers(10, 2);
        assert_eq!(result, MyResult::Ok(5));
        let result2 = divide_numbers(10, 0);
        assert!(matches!(result2, MyResult::Err(_)));
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::{divide_numbers, MyResult};
        let result = divide_numbers(10, 2);
        assert_eq!(result, MyResult::Ok(5), "divide_numbers should perform valid division");
        let result2 = divide_numbers(10, 0);
        assert!(matches!(result2, MyResult::Err(_)), "divide_numbers should return error for division by zero");
    }
}

fn get_source_code() -> &'static str {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        include_str!("solution.rs")
    }
    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        include_str!("../main.rs")
    }
}