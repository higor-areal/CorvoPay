use crate::domain::error::ValidationError;
use crate::domain::value_objects::customer::{
    email::Email, 
    document::{Document, DocumentType},
    name::Name};
use serde::{Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Payer {
    pub email: Email,
    pub name: Name,
    pub document: Document,
}

#[allow(dead_code)]
impl Payer {
    pub fn new(
        email: impl Into<String>,
        name: impl Into<String>,
        document: impl Into<String>,
        document_type: DocumentType,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            email: Email::new(email)?,
            name: Name::new(name)?,
            document: Document::new(document, document_type)?,
        })
    }

    pub fn from_parts(
        email: Email,
        name: Name,
        document: Document,
    ) -> Self {
        Self {
            email,
            name,
            document,
        }
    }
}

use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for Payer {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct PayerDto {
            email: Email,
            name: Name,
            document: Document
        }

        let dto = PayerDto::deserialize(deserializer)?;

        Ok(
            Payer::from_parts(
                dto.email,
                dto.name,
                dto.document,
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- helpers ---
    const EMAIL_VALIDO: &str = "joao@exemplo.com";
    const NOME_VALIDO: &str = "João Silva";
    const CPF_VALIDO: &str = "52998224725";
    const CNPJ_VALIDO: &str = "11222333000181";

    fn email() -> Email {
        Email::new(EMAIL_VALIDO).unwrap()
    }

    fn nome() -> Name {
        Name::new(NOME_VALIDO).unwrap()
    }

    fn documento_cpf() -> Document {
        Document::new(CPF_VALIDO, DocumentType::Cpf).unwrap()
    }
    #[allow(dead_code)]
    fn documento_cnpj() -> Document {
        Document::new(CNPJ_VALIDO, DocumentType::Cnpj).unwrap()
    }

    // -------------------------------------------------------------------------
    mod domain {
        use super::*;

        #[test]
        fn should_create_payer_with_valid_cpf_via_new() {
            let result = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf);
            assert!(result.is_ok());
        }

        #[test]
        fn should_create_payer_with_valid_cnpj_via_new() {
            let result = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CNPJ_VALIDO, DocumentType::Cnpj);
            assert!(result.is_ok());
        }

        #[test]
        fn should_create_payer_via_from_parts() {
            let payer = Payer::from_parts(email(), nome(), documento_cpf());
            assert_eq!(payer.email, email());
        }

        #[test]
        fn should_store_correct_email_after_creation() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            assert_eq!(payer.email.as_str(), EMAIL_VALIDO);
        }

        #[test]
        fn should_store_correct_name_after_creation() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            assert_eq!(payer.name.as_str(), NOME_VALIDO);
        }

        #[test]
        fn should_store_correct_document_number_after_creation() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            assert_eq!(payer.document.number(), CPF_VALIDO);
        }

        #[test]
        fn should_store_correct_document_type_after_creation() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            assert_eq!(payer.document.document_type(), &DocumentType::Cpf);
        }

        #[test]
        fn should_fail_when_email_is_invalid() {
            let result = Payer::new("nao-eh-email", NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_name_is_too_short() {
            let result = Payer::new(EMAIL_VALIDO, "Jo", CPF_VALIDO, DocumentType::Cpf);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_document_number_is_invalid() {
            let result = Payer::new(EMAIL_VALIDO, NOME_VALIDO, "00000000000", DocumentType::Cpf);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_document_type_does_not_match_number() {
            let result = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cnpj);
            assert!(result.is_err());
        }

        #[test]
        fn should_produce_equal_payers_from_new_and_from_parts() {
            let via_new = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            let via_parts = Payer::from_parts(email(), nome(), documento_cpf());
            assert_eq!(via_new, via_parts);
        }
    }

    // -------------------------------------------------------------------------
    mod serialize {
        use super::*;

        #[test]
        fn should_serialize_payer_as_json_object() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            let json = serde_json::to_value(&payer).unwrap();
            assert!(json.is_object());
        }

        #[test]
        fn should_serialize_email_field_correctly() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            let json = serde_json::to_value(&payer).unwrap();
            assert_eq!(json["email"], EMAIL_VALIDO);
        }

        #[test]
        fn should_serialize_name_field_correctly() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            let json = serde_json::to_value(&payer).unwrap();
            assert_eq!(json["name"], NOME_VALIDO);
        }

        #[test]
        fn should_serialize_document_number_as_normalized() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            let json = serde_json::to_value(&payer).unwrap();
            assert_eq!(json["document"]["number"], CPF_VALIDO);
        }

        #[test]
        fn should_serialize_document_type_as_screaming_snake_case() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            let json = serde_json::to_value(&payer).unwrap();
            assert_eq!(json["document"]["document_type"], "CPF");
        }
    }

    // -------------------------------------------------------------------------
    mod deserialize {
        use super::*;

        #[test]
        fn should_deserialize_valid_payer_with_cpf_from_json() {
            let json = format!(
                r#"{{"email":"{}","name":"{}","document":{{"number":"{}","document_type":"cpf"}}}}"#,
                EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO
            );
            let result: Result<Payer, _> = serde_json::from_str(&json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_valid_payer_with_cnpj_from_json() {
            let json = format!(
                r#"{{"email":"{}","name":"{}","document":{{"number":"{}","document_type":"cnpj"}}}}"#,
                EMAIL_VALIDO, NOME_VALIDO, CNPJ_VALIDO
            );
            let result: Result<Payer, _> = serde_json::from_str(&json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_fail_to_deserialize_when_email_field_is_invalid() {
            let json = format!(
                r#"{{"email":"nao-eh-email","name":"{}","document":{{"number":"{}","document_type":"cpf"}}}}"#,
                NOME_VALIDO, CPF_VALIDO
            );
            let result: Result<Payer, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_when_name_is_too_short() {
            let json = format!(
                r#"{{"email":"{}","name":"Jo","document":{{"number":"{}","document_type":"cpf"}}}}"#,
                EMAIL_VALIDO, CPF_VALIDO
            );
            let result: Result<Payer, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_when_document_number_is_invalid() {
            let json = format!(
                r#"{{"email":"{}","name":"{}","document":{{"number":"00000000000","document_type":"cpf"}}}}"#,
                EMAIL_VALIDO, NOME_VALIDO
            );
            let result: Result<Payer, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_when_email_field_is_missing() {
            let json = format!(
                r#"{{"name":"{}","document":{{"number":"{}","document_type":"cpf"}}}}"#,
                NOME_VALIDO, CPF_VALIDO
            );
            let result: Result<Payer, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_when_name_field_is_missing() {
            let json = format!(
                r#"{{"email":"{}","document":{{"number":"{}","document_type":"cpf"}}}}"#,
                EMAIL_VALIDO, CPF_VALIDO
            );
            let result: Result<Payer, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_when_document_field_is_missing() {
            let json = format!(
                r#"{{"email":"{}","name":"{}"}}"#,
                EMAIL_VALIDO, NOME_VALIDO
            );
            let result: Result<Payer, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }
    }

    // -------------------------------------------------------------------------
    mod edge_cases {
        use super::*;

        #[test]
        fn should_normalize_email_to_lowercase_during_creation() {
            let payer = Payer::new("JOAO@EXEMPLO.COM", NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            assert_eq!(payer.email.as_str(), "joao@exemplo.com");
        }

        #[test]
        fn should_trim_name_whitespace_during_creation() {
            let payer = Payer::new(EMAIL_VALIDO, "  João Silva  ", CPF_VALIDO, DocumentType::Cpf).unwrap();
            assert_eq!(payer.name.as_str(), "João Silva");
        }

        #[test]
        fn should_normalize_document_removing_mask_during_creation() {
            let payer = Payer::new(EMAIL_VALIDO, NOME_VALIDO, "529.982.247-25", DocumentType::Cpf).unwrap();
            assert_eq!(payer.document.number(), "52998224725");
        }

        #[test]
        fn should_preserve_all_fields_after_roundtrip_serialization() {
            let original = Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let restored: Payer = serde_json::from_str(&json).unwrap();
            assert_eq!(original, restored);
        }

        #[test]
        fn should_create_equal_payers_regardless_of_cpf_mask() {
            let com_mascara = Payer::new(EMAIL_VALIDO, NOME_VALIDO, "529.982.247-25", DocumentType::Cpf).unwrap();
            let sem_mascara = Payer::new(EMAIL_VALIDO, NOME_VALIDO, "52998224725", DocumentType::Cpf).unwrap();
            assert_eq!(com_mascara, sem_mascara);
        }
    }
}