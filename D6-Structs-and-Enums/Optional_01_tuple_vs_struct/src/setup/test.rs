#[test]
fn test_brew_with_tuple_uses_tuple_indexing() {
    let source_code = get_source_code();
    let has_tuple_access = source_code.contains("potion.0") && source_code.contains("potion.1") && source_code.contains("potion.2");
    assert!(has_tuple_access, "brew_with_tuple should access tuple elements using .0, .1, .2");
}

#[test]
fn test_potion_struct_exists() {
    let source_code = get_source_code();
    let has_struct = source_code.contains("struct Potion");
    assert!(has_struct, "Potion struct should be defined");
}

#[test]
fn test_brew_with_struct_uses_named_fields() {
    let source_code = get_source_code();
    let has_field_access = source_code.contains("potion.volume_ml")
        && source_code.contains("potion.toxicity_level")
        && source_code.contains("potion.color_code");
    assert!(has_field_access, "brew_with_struct should access fields using named field access");
}

#[test]
fn test_potion_struct_field_order_independence() {
    #[cfg(all(feature = "solution", solution_exists))]
    let potion = crate::setup::solution::Potion {
        toxicity_level: 7,
        color_code: String::from("blue"),
        volume_ml: 300,
    };

    #[cfg(not(all(feature = "solution", solution_exists)))]
    let potion = crate::Potion {
        toxicity_level: 7,
        color_code: String::from("blue"),
        volume_ml: 300,
    };

    assert_eq!(potion.volume_ml, 300);
    assert_eq!(potion.toxicity_level, 7);
    assert_eq!(potion.color_code, "blue");
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
