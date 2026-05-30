#[test]
fn test_get_method_used() {
    let source_code = get_source_code();
    assert!(source_code.contains(".get("), "safe_get_sport should use .get() method");
}

#[test]
fn test_iter_and_filter_map_used() {
    let source_code = get_source_code();
    assert!(source_code.contains(".iter("), "Should use .iter() method");
    assert!(source_code.contains(".filter_map(") || source_code.contains(".map("), "Should use .filter_map() or .map() method");
}

#[test]
fn test_position_method_used() {
    let source_code = get_source_code();
    assert!(source_code.contains(".position(") || source_code.contains(".find("), "find_sport_position should use .position() or .find() method");
}

#[test]
fn test_safe_get_sport_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::safe_get_sport;
        use crate::SPORTS;
        let result = safe_get_sport(&SPORTS, 2);
        assert_eq!(result, Some("Tennis".to_string()));
        let result2 = safe_get_sport(&SPORTS, 10);
        assert_eq!(result2, None);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::safe_get_sport;
        use crate::SPORTS;
        let result = safe_get_sport(&SPORTS, 2);
        assert_eq!(result, Some("Tennis".to_string()), "safe_get_sport should return Some for valid indices");
        let result2 = safe_get_sport(&SPORTS, 10);
        assert_eq!(result2, None, "safe_get_sport should return None for invalid indices");
    }
}

#[test]
fn test_get_multiple_sports_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::get_multiple_sports;
        use crate::SPORTS;
        let indices = [0, 2, 4];
        let result = get_multiple_sports(&SPORTS, &indices);
        assert_eq!(result, vec!["Soccer".to_string(), "Tennis".to_string(), "Running".to_string()]);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::get_multiple_sports;
        use crate::SPORTS;
        let indices = [0, 2, 4];
        let result = get_multiple_sports(&SPORTS, &indices);
        assert_eq!(result, vec!["Soccer".to_string(), "Tennis".to_string(), "Running".to_string()], "get_multiple_sports should return all valid sports");
    }
}

#[test]
fn test_find_sport_position_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::find_sport_position;
        use crate::SPORTS;
        let result = find_sport_position(&SPORTS, "Tennis");
        assert_eq!(result, Some(2));
        let result2 = find_sport_position(&SPORTS, "Golf");
        assert_eq!(result2, None);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::find_sport_position;
        use crate::SPORTS;
        let result = find_sport_position(&SPORTS, "Tennis");
        assert_eq!(result, Some(2), "find_sport_position should find existing sports");
        let result2 = find_sport_position(&SPORTS, "Golf");
        assert_eq!(result2, None, "find_sport_position should return None for non-existing sports");
    }
}

#[test]
fn test_get_score_range_functionality() {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        use crate::setup::solution::get_score_range;
        use crate::SCORES;
        let result = get_score_range(&SCORES, 1, 3);
        assert_eq!(result, vec![92, 78]);
    }

    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        use crate::get_score_range;
        use crate::SCORES;
        let result = get_score_range(&SCORES, 1, 3);
        assert_eq!(result, vec![92, 78], "get_score_range should return correct range");
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