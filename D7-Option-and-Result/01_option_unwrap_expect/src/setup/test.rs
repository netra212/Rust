#[test]
fn test_unwrap_method_used() {
    let source_code = get_source_code();
    assert!(source_code.contains(".unwrap("), "get_score_with_panic should use .unwrap() method");
}

#[test]
fn test_expect_method_used() {
    let source_code = get_source_code();
    assert!(source_code.contains(".expect("), "Should use .expect() method with custom message");
}

#[test]
fn test_unwrap_or_method_used() {
    let source_code = get_source_code();
    assert!(source_code.contains(".unwrap_or("), "get_score_or_default should use .unwrap_or() method");
}

#[test]
fn test_get_score_with_panic_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::get_score_with_panic;
        use crate::SCORES;
        let result = get_score_with_panic(&SCORES, 0);
        assert_eq!(result, 85);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::get_score_with_panic;
        use crate::SCORES;
        let result = get_score_with_panic(&SCORES, 0);
        assert_eq!(result, 85, "get_score_with_panic should extract Some values correctly");
    }
}

#[test]
fn test_get_score_with_message_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::get_score_with_message;
        use crate::SCORES;
        let result = get_score_with_message(&SCORES, 2);
        assert_eq!(result, 92);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::get_score_with_message;
        use crate::SCORES;
        let result = get_score_with_message(&SCORES, 2);
        assert_eq!(result, 92, "get_score_with_message should extract Some values correctly");
    }
}

#[test]
fn test_get_score_or_default_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::get_score_or_default;
        use crate::SCORES;
        let result = get_score_or_default(&SCORES, 1, 50);
        assert_eq!(result, 50);
        let result2 = get_score_or_default(&SCORES, 0, 50);
        assert_eq!(result2, 85);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::get_score_or_default;
        use crate::SCORES;
        let result = get_score_or_default(&SCORES, 1, 50);
        assert_eq!(result, 50, "get_score_or_default should return default for None values");
        let result2 = get_score_or_default(&SCORES, 0, 50);
        assert_eq!(result2, 85, "get_score_or_default should return actual value for Some");
    }
}

#[test]
fn test_parse_number_or_panic_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::parse_number_or_panic;
        use crate::USER_INPUTS;
        let result = parse_number_or_panic(USER_INPUTS[0]);
        assert_eq!(result, 42);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::parse_number_or_panic;
        use crate::USER_INPUTS;
        let result = parse_number_or_panic(USER_INPUTS[0]);
        assert_eq!(result, 42, "parse_number_or_panic should parse valid number strings");
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