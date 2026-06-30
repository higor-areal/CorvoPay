use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GatewayAttemptId(Uuid);

#[allow(dead_code)]
impl GatewayAttemptId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const UUID_V7_VALIDO: &str = "01932b4a-f1c3-7a2e-9b1d-3f4e5a6b7c8d";
    const UUID_V4_VALIDO: &str = "550e8400-e29b-41d4-a716-446655440000";

    mod serialize {
        use super::*;

        #[test]
        fn should_serialize_as_transparent_uuid_string() {
            let id: GatewayAttemptId = serde_json::from_str(&format!("\"{}\"", UUID_V7_VALIDO)).unwrap();
            let json = serde_json::to_string(&id).unwrap();
            assert_eq!(json, format!("\"{}\"", UUID_V7_VALIDO));
        }

        #[test]
        fn should_not_serialize_as_object_with_field() {
            let id: GatewayAttemptId = serde_json::from_str(&format!("\"{}\"", UUID_V7_VALIDO)).unwrap();
            let json = serde_json::to_string(&id).unwrap();
            assert!(!json.contains('{'));
        }
    }

    mod deserialize {
        use super::*;

        #[test]
        fn should_deserialize_valid_uuid_v7_string() {
            let result: Result<GatewayAttemptId, _> =
                serde_json::from_str(&format!("\"{}\"", UUID_V7_VALIDO));
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_valid_uuid_v4_string() {
            // a struct aceita qualquer UUID válido — a versão é garantida pelo banco
            let result: Result<GatewayAttemptId, _> =
                serde_json::from_str(&format!("\"{}\"", UUID_V4_VALIDO));
            assert!(result.is_ok());
        }

        #[test]
        fn should_return_correct_uuid_after_deserialization() {
            let id: GatewayAttemptId =
                serde_json::from_str(&format!("\"{}\"", UUID_V7_VALIDO)).unwrap();
            assert_eq!(id.as_uuid().to_string(), UUID_V7_VALIDO);
        }

        #[test]
        fn should_fail_to_deserialize_invalid_uuid_string() {
            let result: Result<GatewayAttemptId, _> = serde_json::from_str("\"nao-eh-um-uuid\"");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_empty_string() {
            let result: Result<GatewayAttemptId, _> = serde_json::from_str("\"\"");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_number_instead_of_string() {
            let result: Result<GatewayAttemptId, _> = serde_json::from_str("123");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_null() {
            let result: Result<GatewayAttemptId, _> = serde_json::from_str("null");
            assert!(result.is_err());
        }

        #[test]
        fn should_deserialize_uuid_without_hyphens() {
            // a crate uuid com feature serde aceita o formato compacto (sem hífens)
            let result: Result<GatewayAttemptId, _> =
                serde_json::from_str("\"01932b4af1c37a2e9b1d3f4e5a6b7c8d\"");
            assert!(result.is_ok());
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn should_preserve_uuid_value_after_roundtrip_serialization() {
            let original: GatewayAttemptId =
                serde_json::from_str(&format!("\"{}\"", UUID_V7_VALIDO)).unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let restored: GatewayAttemptId = serde_json::from_str(&json).unwrap();
            assert_eq!(original.as_uuid(), restored.as_uuid());
        }

        #[test]
        fn should_serialize_uuid_in_lowercase_hyphenated_format() {
            let id: GatewayAttemptId =
                serde_json::from_str(&format!("\"{}\"", UUID_V7_VALIDO)).unwrap();
            let json = serde_json::to_string(&id).unwrap();
            assert!(json.chars().all(|c| !c.is_ascii_uppercase()));
        }
    }
}