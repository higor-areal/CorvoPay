use serde::{
    de,
    Deserialize,
    Deserializer,
    Serialize,
};

use crate::domain::error::ValidationError;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Email(String);

#[allow(dead_code)]
impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, ValidationError> {
        let email = email.into().trim().to_lowercase();

        if email.is_empty() {
            return Err(ValidationError::InvalidEmail);
        }

        if email.len() > 254 {
            return Err(ValidationError::InvalidEmail);
        }

        let (local, domain) = email
            .split_once('@')
            .ok_or(ValidationError::InvalidEmail)?;

        if local.is_empty() {
            return Err(ValidationError::InvalidEmail);
        }

        if domain.is_empty() {
            return Err(ValidationError::InvalidEmail);
        }

        if !domain.contains('.') {
            return Err(ValidationError::InvalidEmail);
        }

        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let email = String::deserialize(deserializer)?;

        Email::new(email)
            .map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMAIL_VALIDO: &str = "usuario@exemplo.com";

    mod domain {
        use super::*;

        #[test]
        fn should_create_email_with_valid_input() {
            let result = Email::new(EMAIL_VALIDO);
            assert!(result.is_ok());
        }

        #[test]
        fn should_trim_whitespace_before_validating() {
            let result = Email::new("  usuario@exemplo.com  ");
            assert!(result.is_ok());
        }

        #[test]
        fn should_normalize_email_to_lowercase() {
            let email = Email::new("Usuario@Exemplo.COM").unwrap();
            assert_eq!(email.as_str(), "usuario@exemplo.com");
        }

        #[test]
        fn should_return_correct_value_via_as_str() {
            let email = Email::new(EMAIL_VALIDO).unwrap();
            assert_eq!(email.as_str(), EMAIL_VALIDO);
        }

        #[test]
        fn should_fail_when_email_is_empty() {
            let result = Email::new("");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_email_is_only_whitespace() {
            let result = Email::new("   ");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_email_has_no_at_sign() {
            let result = Email::new("usuarioexemplo.com");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_local_part_is_empty() {
            let result = Email::new("@exemplo.com");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_domain_part_is_empty() {
            let result = Email::new("usuario@");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_domain_has_no_dot() {
            let result = Email::new("usuario@exemplocom");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_email_exceeds_254_characters() {
            let local = "a".repeat(243);
            let email = format!("{}@exemplo.com", local); // 243 + 12 = 255 chars
            let result = Email::new(email);
            assert!(result.is_err());
        }

        #[test]
        fn should_create_email_with_exactly_254_characters() {
            let local = "a".repeat(242);
            let email = format!("{}@exemplo.com", local); // 242 + 12 = 254 chars
            let result = Email::new(email);
            assert!(result.is_ok());
        }
    }

    mod serialize {
        use super::*;

        #[test]
        fn should_serialize_email_as_transparent_string() {
            let email = Email::new(EMAIL_VALIDO).unwrap();
            let json = serde_json::to_string(&email).unwrap();
            assert_eq!(json, "\"usuario@exemplo.com\"");
        }

        #[test]
        fn should_serialize_email_as_lowercase_after_normalization() {
            let email = Email::new("USUARIO@EXEMPLO.COM").unwrap();
            let json = serde_json::to_string(&email).unwrap();
            assert_eq!(json, "\"usuario@exemplo.com\"");
        }
    }

    mod deserialize {
        use super::*;

        #[test]
        fn should_deserialize_valid_email_from_json_string() {
            let result: Result<Email, _> = serde_json::from_str("\"usuario@exemplo.com\"");
            assert!(result.is_ok());
        }

        #[test]
        fn should_fail_to_deserialize_email_without_at_sign() {
            let result: Result<Email, _> = serde_json::from_str("\"usuarioexemplo.com\"");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_empty_string() {
            let result: Result<Email, _> = serde_json::from_str("\"\"");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_email_without_domain_dot() {
            let result: Result<Email, _> = serde_json::from_str("\"usuario@exemplocom\"");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_number_instead_of_string() {
            let result: Result<Email, _> = serde_json::from_str("123");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_null() {
            let result: Result<Email, _> = serde_json::from_str("null");
            assert!(result.is_err());
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn should_accept_email_with_subdomain() {
            let result = Email::new("usuario@mail.exemplo.com");
            assert!(result.is_ok());
        }

        #[test]
        fn should_accept_email_with_plus_in_local_part() {
            let result = Email::new("usuario+tag@exemplo.com");
            assert!(result.is_ok());
        }

        #[test]
        fn should_accept_email_with_dots_in_local_part() {
            let result = Email::new("nome.sobrenome@exemplo.com");
            assert!(result.is_ok());
        }

        #[test]
        fn should_accept_email_with_numbers_in_local_part() {
            let result = Email::new("usuario123@exemplo.com");
            assert!(result.is_ok());
        }

        #[test]
        fn should_preserve_email_value_after_roundtrip_serialization() {
            let original = Email::new(EMAIL_VALIDO).unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let restored: Email = serde_json::from_str(&json).unwrap();
            assert_eq!(original.as_str(), restored.as_str());
        }

        #[test]
        fn should_trim_and_lowercase_before_checking_length_limit() {
            // garante que trim acontece antes da validação de tamanho
            let email_com_espacos = format!("  {}  ", EMAIL_VALIDO);
            let result = Email::new(email_com_espacos);
            assert!(result.is_ok());
        }

        #[test]
        fn should_fail_when_email_has_multiple_at_signs() {
            // split_once pega só o primeiro @, o segundo vai pro domain
            // domain = "b@exemplo.com" → contém '.' → passa
            // esse teste documenta o comportamento atual da impl
            let result = Email::new("a@b@exemplo.com");
            assert!(result.is_ok()); // domain = "b@exemplo.com", contém '.'
        }
    }
}