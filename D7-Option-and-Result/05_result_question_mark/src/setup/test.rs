#[test]
fn test_question_mark_usage() {
    let source_code = get_source_code();
    assert!(source_code.contains("split_once(") || source_code.contains("split(") || source_code.contains("strip_prefix("), "Should use string splitting or strip_prefix");
    assert!(source_code.contains("parse(") || source_code.contains(".parse::"), "Should use parse method");
}

#[test]
fn test_parse_user_name_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::parse_user_name;
        let result = parse_user_name("name:Alice");
        assert_eq!(result, Ok("Alice".to_string()));
        let result2 = parse_user_name("invalid");
        assert!(result2.is_err());
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::parse_user_name;
        let result = parse_user_name("name:Alice");
        assert_eq!(result, Ok("Alice".to_string()), "parse_user_name should parse valid format");
        let result2 = parse_user_name("invalid");
        assert!(result2.is_err(), "parse_user_name should return error for invalid format");
    }
}

#[test]
fn test_parse_user_age_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::parse_user_age;
        let result = parse_user_age("age:25");
        assert_eq!(result, Ok(25));
        let result2 = parse_user_age("age:abc");
        assert!(result2.is_err());
        let result3 = parse_user_age("invalid");
        assert!(result3.is_err());
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::parse_user_age;
        let result = parse_user_age("age:25");
        assert_eq!(result, Ok(25), "parse_user_age should parse valid age");
        let result2 = parse_user_age("age:abc");
        assert!(result2.is_err(), "parse_user_age should return error for invalid number");
        let result3 = parse_user_age("invalid");
        assert!(result3.is_err(), "parse_user_age should return error for invalid format");
    }
}

#[test]
fn test_parse_user_email_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::parse_user_email;
        let result = parse_user_email("email:alice@example.com");
        assert_eq!(result, Ok("alice@example.com".to_string()));
        let result2 = parse_user_email("email:invalid");
        assert!(result2.is_err());
        let result3 = parse_user_email("invalid");
        assert!(result3.is_err());
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::parse_user_email;
        let result = parse_user_email("email:alice@example.com");
        assert_eq!(result, Ok("alice@example.com".to_string()), "parse_user_email should parse valid email");
        let result2 = parse_user_email("email:invalid");
        assert!(result2.is_err(), "parse_user_email should return error for email without @");
        let result3 = parse_user_email("invalid");
        assert!(result3.is_err(), "parse_user_email should return error for invalid format");
    }
}

#[test]
fn test_parse_complete_user_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{parse_complete_user, User};
        let result = parse_complete_user("name:Bob", "age:30", "email:bob@test.com");
        assert_eq!(result, Ok(User {
            name: "Bob".to_string(),
            age: 30,
            email: "bob@test.com".to_string(),
        }));
        let result2 = parse_complete_user("invalid", "age:30", "email:bob@test.com");
        assert!(result2.is_err());
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::{parse_complete_user, User};
        let result = parse_complete_user("name:Bob", "age:30", "email:bob@test.com");
        assert_eq!(result, Ok(User {
            name: "Bob".to_string(),
            age: 30,
            email: "bob@test.com".to_string(),
        }), "parse_complete_user should create valid User");
        let result2 = parse_complete_user("invalid", "age:30", "email:bob@test.com");
        assert!(result2.is_err(), "parse_complete_user should propagate errors");
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