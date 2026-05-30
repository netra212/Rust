#[test]
fn test_my_option_enum_exists() {
    let source_code = get_source_code();
    let has_enum = source_code.contains("enum MyOption");
    assert!(has_enum, "MyOption enum should be defined");
}

#[test]
fn test_my_option_has_some_and_none_variants() {
    let source_code = get_source_code();
    let has_some = source_code.contains("Some(T)");
    let has_none = source_code.contains("None");
    assert!(has_some && has_none, "MyOption should have Some(T) and None variants");
}

#[test]
fn test_get_snack_function_exists() {
    let source_code = get_source_code();
    let has_function = source_code.contains("fn get_snack");
    assert!(has_function, "get_snack function should be defined");
}

#[test]
fn test_get_snack_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::{get_snack, MyOption};

        let machine = ["chips", "cookie", "water"];

        // Test finding an existing snack
        let result = get_snack("cookie", machine);
        match result {
            MyOption::Some(snack) => assert_eq!(snack, "cookie"),
            MyOption::None => panic!("Expected Some(cookie), got None"),
        }

        // Test not finding a snack
        let result2 = get_snack("soda", machine);
        match result2 {
            MyOption::Some(_) => panic!("Expected None, got Some"),
            MyOption::None => {},
        }
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
