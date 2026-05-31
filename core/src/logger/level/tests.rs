use std::str::FromStr;

use crate::logger::level::Level;

static LEVELS: &[(Level, &str)] = &[
    (Level::DEBUG, "DEBUG"),
    (Level::INFO, "INFO"),
    (Level::WARNING, "WARN"),
    (Level::ERROR, "ERROR"),
    (Level::CRITICAL, "CRIT"),
];

#[test]
fn test_01_parse_valid_level_uppercase() {
    LEVELS.iter().for_each(|(level, literal)| {
        let result = Level::from_str(literal);
        assert!(result.is_ok(), "Failed to parse: {}", literal);
        let res_level = result.unwrap();
        assert!(
            *level == res_level,
            "Not matching level {:?} with literal {}",
            level,
            literal
        )
    });
}

#[test]
fn test_02_parse_valid_level_lowercase() {
    LEVELS.iter().for_each(|(level, literal)| {
        let literal = &literal.to_lowercase();
        let result = Level::from_str(literal);
        assert!(result.is_ok(), "Failed to parse: {}", literal);
        let res_level = result.unwrap();
        assert!(
            *level == res_level,
            "Not matching level {:?} with literal {}",
            level,
            literal
        )
    });
}

#[test]
fn test_03_level_to_string() {
    LEVELS
        .iter()
        .for_each(|(level, literal)| assert!(level.to_string() == *literal));
}
