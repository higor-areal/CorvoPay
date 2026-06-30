use serde::{
    de,
    Deserialize,
    Deserializer,
    Serialize,
};

use crate::domain::error::ValidationError;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Name(String);

#[allow(dead_code)]
impl Name {
    pub fn new(name: impl Into<String>) -> Result<Self, ValidationError> {
        let name = name.into().trim().to_string();

        const MIN_LENGTH: usize = 3;
        const MAX_LENGTH: usize = 120;

        if name.len() < MIN_LENGTH || name.len() > MAX_LENGTH {
            return Err(ValidationError::InvalidName);
        }

        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;

        Name::new(name)
            .map_err(de::Error::custom)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const NAME_VALIDO: &str = "João Silva";

    mod domain {
        use super::*;

        #[test]
        fn should_create_name_with_valid_input() {
            let result = Name::new(NAME_VALIDO);
            assert!(result.is_ok());
        }

        #[test]
        fn should_create_name_with_exactly_minimum_length() {
            let result = Name::new("Ana"); // 3 chars
            assert!(result.is_ok());
        }

        #[test]
        fn should_create_name_with_exactly_maximum_length() {
            let name = "a".repeat(120);
            let result = Name::new(name);
            assert!(result.is_ok());
        }

        #[test]
        fn should_trim_whitespace_before_validating() {
            let result = Name::new("  João Silva  ");
            assert!(result.is_ok());
        }

        #[test]
        fn should_preserve_trimmed_value_via_as_str() {
            let name = Name::new("  João Silva  ").unwrap();
            assert_eq!(name.as_str(), "João Silva");
        }

        #[test]
        fn should_return_correct_value_via_as_str() {
            let name = Name::new(NAME_VALIDO).unwrap();
            assert_eq!(name.as_str(), NAME_VALIDO);
        }

        #[test]
        fn should_fail_when_name_is_empty() {
            let result = Name::new("");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_name_is_only_whitespace() {
            let result = Name::new("   ");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_name_is_below_minimum_length() {
            let result = Name::new("Jo"); // 2 chars
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_name_exceeds_maximum_length() {
            let name = "a".repeat(121);
            let result = Name::new(name);
            assert!(result.is_err());
        }
    }

    mod serialize {
        use super::*;

        #[test]
        fn should_serialize_name_as_transparent_string() {
            let name = Name::new(NAME_VALIDO).unwrap();
            let json = serde_json::to_string(&name).unwrap();
            assert_eq!(json, "\"João Silva\"");
        }

        #[test]
        fn should_serialize_trimmed_value_not_original_input() {
            let name = Name::new("  João Silva  ").unwrap();
            let json = serde_json::to_string(&name).unwrap();
            assert_eq!(json, "\"João Silva\"");
        }
    }

    mod deserialize {
        use super::*;

        #[test]
        fn should_deserialize_valid_name_from_json_string() {
            let result: Result<Name, _> = serde_json::from_str("\"João Silva\"");
            assert!(result.is_ok());
        }

        #[test]
        fn should_fail_to_deserialize_name_below_minimum_length() {
            let result: Result<Name, _> = serde_json::from_str("\"Jo\"");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_empty_string() {
            let result: Result<Name, _> = serde_json::from_str("\"\"");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_name_exceeding_maximum_length() {
            let name = "a".repeat(121);
            let json = format!("\"{}\"", name);
            let result: Result<Name, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_number_instead_of_string() {
            let result: Result<Name, _> = serde_json::from_str("123");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_null() {
            let result: Result<Name, _> = serde_json::from_str("null");
            assert!(result.is_err());
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn should_fail_when_name_has_two_chars_after_trim() {
            let result = Name::new("  Jo  "); // trim → "Jo" = 2 chars
            assert!(result.is_err());
        }

        #[test]
        fn should_accept_name_with_exactly_three_chars_after_trim() {
            let result = Name::new("  Ana  "); // trim → "Ana" = 3 chars
            assert!(result.is_ok());
        }

        #[test]
        fn should_accept_name_with_special_characters() {
            let result = Name::new("Ângela Conceição");
            assert!(result.is_ok());
        }

        #[test]
        fn should_accept_name_with_hyphens() {
            let result = Name::new("Ana-Clara");
            assert!(result.is_ok());
        }

        #[test]
        fn should_preserve_value_after_roundtrip_serialization() {
            let original = Name::new(NAME_VALIDO).unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let restored: Name = serde_json::from_str(&json).unwrap();
            assert_eq!(original.as_str(), restored.as_str());
        }

        #[test]
        fn should_measure_length_after_trim_not_before() {
            // "  a  " tem 5 chars brutos mas 1 após trim → deve falhar
            let result = Name::new("  a  ");
            assert!(result.is_err());
        }
    }
}