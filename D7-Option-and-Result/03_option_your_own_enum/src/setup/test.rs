#[test]
fn test_pattern_matching_used() {
    let source_code = get_source_code();
    assert!(source_code.contains("match ") || source_code.contains("if let"), "Should use pattern matching");
    assert!(source_code.contains("MyOption::Some"), "Should construct MyOption::Some");
    assert!(source_code.contains("MyOption::None"), "Should handle MyOption::None");
}

#[test]
fn test_create_some_value_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::create_some_value;
        let result = create_some_value(42);
        assert_eq!(result, crate::setup::solution::MyOption::Some(42));
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::create_some_value;
        let result = create_some_value(42);
        assert_eq!(result, crate::MyOption::Some(42), "create_some_value should create Some variant");
    }
}

#[test]
fn test_double_if_some_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{double_if_some, MyOption};
        let result = double_if_some(MyOption::Some(5));
        assert_eq!(result, MyOption::Some(10));
        let result2 = double_if_some(MyOption::None);
        assert_eq!(result2, MyOption::None);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::{double_if_some, MyOption};
        let result = double_if_some(MyOption::Some(5));
        assert_eq!(result, MyOption::Some(10), "double_if_some should double Some values");
        let result2 = double_if_some(MyOption::None);
        assert_eq!(result2, MyOption::None, "double_if_some should preserve None");
    }
}

#[test]
fn test_combine_options_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{combine_options, MyOption};
        let result = combine_options(MyOption::Some(3), MyOption::Some(7));
        assert_eq!(result, MyOption::Some(10));
        let result2 = combine_options(MyOption::Some(3), MyOption::None);
        assert_eq!(result2, MyOption::None);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::{combine_options, MyOption};
        let result = combine_options(MyOption::Some(3), MyOption::Some(7));
        assert_eq!(result, MyOption::Some(10), "combine_options should add two Some values");
        let result2 = combine_options(MyOption::Some(3), MyOption::None);
        assert_eq!(result2, MyOption::None, "combine_options should return None if either is None");
    }
}

#[test]
fn test_convert_to_string_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{convert_to_string, MyOption};
        let result = convert_to_string(MyOption::Some(42));
        assert_eq!(result, MyOption::Some("42".to_string()));
        let result2 = convert_to_string(MyOption::None);
        assert_eq!(result2, MyOption::None);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::{convert_to_string, MyOption};
        let result = convert_to_string(MyOption::Some(42));
        assert_eq!(result, MyOption::Some("42".to_string()), "convert_to_string should convert Some values to strings");
        let result2 = convert_to_string(MyOption::None);
        assert_eq!(result2, MyOption::None, "convert_to_string should preserve None");
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