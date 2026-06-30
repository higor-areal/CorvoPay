use serde::{
    Serialize,
    Deserialize,
    Deserializer,
    de,
};
use crate::domain::error::ValidationError;


#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentType {
    Cpf,
    Cnpj,
}

impl DocumentType {
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "cpf" => Ok(Self::Cpf),
            "cnpj" => Ok(Self::Cnpj),
            _ => Err(ValidationError::InvalidDocument),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Document {
    number: String,
    document_type: DocumentType,
}


impl<'de> Deserialize<'de> for DocumentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        Self::new(&value).map_err(serde::de::Error::custom)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct DocumentRepr {
    number: String,
    document_type: DocumentType,
}

#[allow(dead_code)]
impl<'de> Deserialize<'de> for Document {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = DocumentRepr::deserialize(deserializer)?;
        Self::new(repr.number, repr.document_type)
            .map_err(de::Error::custom)
    }
}

#[allow(dead_code)]
impl Document {
    pub fn number(&self) -> &str {
        &self.number
    }

    pub fn document_type(&self) -> &DocumentType {
        &self.document_type
    }
}

#[allow(dead_code)]
impl Document {
    pub fn new(
        number: impl Into<String>,
        document_type: DocumentType,
    ) -> Result<Self, ValidationError> {
        let number = number.into();

        match document_type {
            DocumentType::Cpf => validar_cpf(&number)?,
            DocumentType::Cnpj => validar_cnpj(&number)?,
        }

        Ok(Self {
            number: Self::normalize(&number),
            document_type,
        })
    }

    fn normalize(document: &str) -> String {
        document
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect()
    }
}


pub fn validar_cpf(cpf: &str) -> Result<(), ValidationError> {
    let cpf = somente_numeros(cpf);

    if cpf.len() != 11 {
        return Err(ValidationError::InvalidDocument);
    }

    if todos_iguais(&cpf) {
        return Err(ValidationError::InvalidDocument);
    }

    let digits = str_para_digits(&cpf)?;

    let dv1 = calcular_digito(&digits[..9], 10);
    let dv2 = calcular_digito(&digits[..10], 11);

    if digits[9] != dv1 || digits[10] != dv2 {
        return Err(ValidationError::InvalidDocument);
    }

    Ok(())
}

pub fn validar_cnpj(cnpj: &str) -> Result<(), ValidationError> {
    let cnpj = somente_numeros(cnpj);

    if cnpj.len() != 14 {
        return Err(ValidationError::InvalidDocument);
    }

    if todos_iguais(&cnpj) {
        return Err(ValidationError::InvalidDocument);
    }

    let digits = str_para_digits(&cnpj)?;

    let pesos_dv1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let pesos_dv2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let soma1: u32 = digits[..12]
        .iter()
        .zip(pesos_dv1.iter())
        .map(|(d, p)| d * p)
        .sum();

    let dv1 = match soma1 % 11 {
        0 | 1 => 0,
        resto => 11 - resto,
    };

    let soma2: u32 = digits[..13]
        .iter()
        .zip(pesos_dv2.iter())
        .map(|(d, p)| d * p)
        .sum();

    let dv2 = match soma2 % 11 {
        0 | 1 => 0,
        resto => 11 - resto,
    };

    if digits[12] != dv1 || digits[13] != dv2 {
        return Err(ValidationError::InvalidDocument);
    }

    Ok(())
}

fn somente_numeros(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}

fn todos_iguais(input: &str) -> bool {
    input
        .chars()
        .all(|c| c == input.chars().next().unwrap())
}

fn str_para_digits(input: &str) -> Result<Vec<u32>, ValidationError> {
    input
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(ValidationError::InvalidDocument)
        })
        .collect()
}

fn calcular_digito(base: &[u32], peso_inicial: u32) -> u32 {
    let soma: u32 = base
        .iter()
        .zip((2..=peso_inicial).rev())
        .map(|(d, p)| d * p)
        .sum();

    let resto = soma % 11;

    if resto < 2 {
        0
    } else {
        11 - resto
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // CPFs válidos para reuso nos testes
    const CPF_VALIDO: &str = "529.982.247-25";
    const CPF_VALIDO_SEM_MASCARA: &str = "52998224725";
    const CNPJ_VALIDO: &str = "11.222.333/0001-81";
    const CNPJ_VALIDO_SEM_MASCARA: &str = "11222333000181";

    // -------------------------------------------------------------------------
    mod document_type {
        use super::*;

        mod domain {
            use super::*;

            #[test]
            fn should_create_cpf_type_from_lowercase() {
                let result = DocumentType::new("cpf");
                assert!(result.is_ok());
            }

            #[test]
            fn should_create_cnpj_type_from_lowercase() {
                let result = DocumentType::new("cnpj");
                assert!(result.is_ok());
            }

            #[test]
            fn should_create_cpf_type_from_uppercase() {
                let result = DocumentType::new("CPF");
                assert!(result.is_ok());
            }

            #[test]
            fn should_create_cnpj_type_from_uppercase() {
                let result = DocumentType::new("CNPJ");
                assert!(result.is_ok());
            }

            #[test]
            fn should_create_cpf_type_from_mixed_case() {
                let result = DocumentType::new("Cpf");
                assert!(result.is_ok());
            }

            #[test]
            fn should_create_document_type_ignoring_surrounding_whitespace() {
                let result = DocumentType::new("  cpf  ");
                assert!(result.is_ok());
            }

            #[test]
            fn should_fail_when_value_is_unknown_string() {
                let result = DocumentType::new("rg");
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_value_is_empty_string() {
                let result = DocumentType::new("");
                assert!(result.is_err());
            }

            #[test]
            fn should_return_cpf_variant_when_input_is_cpf() {
                let doc_type = DocumentType::new("cpf").unwrap();
                assert_eq!(doc_type, DocumentType::Cpf);
            }

            #[test]
            fn should_return_cnpj_variant_when_input_is_cnpj() {
                let doc_type = DocumentType::new("cnpj").unwrap();
                assert_eq!(doc_type, DocumentType::Cnpj);
            }
        }

        mod serialize {
            use super::*;

            #[test]
            fn should_serialize_cpf_as_screaming_snake_case() {
                let doc_type = DocumentType::Cpf;
                let json = serde_json::to_string(&doc_type).unwrap();
                assert_eq!(json, "\"CPF\"");
            }

            #[test]
            fn should_serialize_cnpj_as_screaming_snake_case() {
                let doc_type = DocumentType::Cnpj;
                let json = serde_json::to_string(&doc_type).unwrap();
                assert_eq!(json, "\"CNPJ\"");
            }
        }

        mod deserialize {
            use super::*;

            #[test]
            fn should_deserialize_cpf_from_lowercase_string() {
                let result: Result<DocumentType, _> = serde_json::from_str("\"cpf\"");
                assert!(result.is_ok());
            }

            #[test]
            fn should_deserialize_cnpj_from_lowercase_string() {
                let result: Result<DocumentType, _> = serde_json::from_str("\"cnpj\"");
                assert!(result.is_ok());
            }

            #[test]
            fn should_deserialize_cpf_from_uppercase_string() {
                let result: Result<DocumentType, _> = serde_json::from_str("\"CPF\"");
                assert!(result.is_ok());
            }

            #[test]
            fn should_fail_to_deserialize_unknown_document_type() {
                let result: Result<DocumentType, _> = serde_json::from_str("\"RG\"");
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_to_deserialize_number_as_document_type() {
                let result: Result<DocumentType, _> = serde_json::from_str("1");
                assert!(result.is_err());
            }
        }
    }

    // -------------------------------------------------------------------------
    mod document {
        use super::*;

        mod domain {
            use super::*;

            #[test]
            fn should_create_document_with_valid_cpf_masked() {
                let result = Document::new(CPF_VALIDO, DocumentType::Cpf);
                assert!(result.is_ok());
            }

            #[test]
            fn should_create_document_with_valid_cpf_unmasked() {
                let result = Document::new(CPF_VALIDO_SEM_MASCARA, DocumentType::Cpf);
                assert!(result.is_ok());
            }

            #[test]
            fn should_create_document_with_valid_cnpj_masked() {
                let result = Document::new(CNPJ_VALIDO, DocumentType::Cnpj);
                assert!(result.is_ok());
            }

            #[test]
            fn should_create_document_with_valid_cnpj_unmasked() {
                let result = Document::new(CNPJ_VALIDO_SEM_MASCARA, DocumentType::Cnpj);
                assert!(result.is_ok());
            }

            #[test]
            fn should_normalize_cpf_removing_punctuation() {
                let doc = Document::new(CPF_VALIDO, DocumentType::Cpf).unwrap();
                assert_eq!(doc.number(), CPF_VALIDO_SEM_MASCARA);
            }

            #[test]
            fn should_normalize_cnpj_removing_punctuation() {
                let doc = Document::new(CNPJ_VALIDO, DocumentType::Cnpj).unwrap();
                assert_eq!(doc.number(), CNPJ_VALIDO_SEM_MASCARA);
            }

            #[test]
            fn should_return_correct_document_type_cpf() {
                let doc = Document::new(CPF_VALIDO, DocumentType::Cpf).unwrap();
                assert_eq!(doc.document_type(), &DocumentType::Cpf);
            }

            #[test]
            fn should_return_correct_document_type_cnpj() {
                let doc = Document::new(CNPJ_VALIDO, DocumentType::Cnpj).unwrap();
                assert_eq!(doc.document_type(), &DocumentType::Cnpj);
            }

            #[test]
            fn should_fail_when_cpf_has_wrong_length() {
                let result = Document::new("123.456.789", DocumentType::Cpf);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_cpf_has_all_same_digits() {
                let result = Document::new("111.111.111-11", DocumentType::Cpf);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_cpf_has_invalid_check_digits() {
                let result = Document::new("529.982.247-00", DocumentType::Cpf);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_cnpj_has_wrong_length() {
                let result = Document::new("11.222.333/0001", DocumentType::Cnpj);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_cnpj_has_all_same_digits() {
                let result = Document::new("11111111111111", DocumentType::Cnpj);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_cnpj_has_invalid_check_digits() {
                let result = Document::new("11.222.333/0001-00", DocumentType::Cnpj);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_cpf_number_is_passed_with_cnpj_type() {
                let result = Document::new(CPF_VALIDO, DocumentType::Cnpj);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_cnpj_number_is_passed_with_cpf_type() {
                let result = Document::new(CNPJ_VALIDO, DocumentType::Cpf);
                assert!(result.is_err());
            }
        }

        mod serialize {
            use super::*;

            #[test]
            fn should_serialize_document_with_normalized_number() {
                let doc = Document::new(CPF_VALIDO, DocumentType::Cpf).unwrap();
                let json = serde_json::to_value(&doc).unwrap();
                assert_eq!(json["number"], CPF_VALIDO_SEM_MASCARA);
            }

            #[test]
            fn should_serialize_document_type_as_screaming_snake_case() {
                let doc = Document::new(CPF_VALIDO, DocumentType::Cpf).unwrap();
                let json = serde_json::to_value(&doc).unwrap();
                assert_eq!(json["document_type"], "CPF");
            }

            #[test]
            fn should_serialize_cnpj_document_type_as_screaming_snake_case() {
                let doc = Document::new(CNPJ_VALIDO, DocumentType::Cnpj).unwrap();
                let json = serde_json::to_value(&doc).unwrap();
                assert_eq!(json["document_type"], "CNPJ");
            }
        }

        mod deserialize {
            use super::*;

            #[test]
            fn should_deserialize_valid_cpf_document_from_json() {
                let json = format!(
                    r#"{{"number": "{}", "document_type": "cpf"}}"#,
                    CPF_VALIDO_SEM_MASCARA
                );
                let result: Result<Document, _> = serde_json::from_str(&json);
                assert!(result.is_ok());
            }

            #[test]
            fn should_deserialize_valid_cnpj_document_from_json() {
                let json = format!(
                    r#"{{"number": "{}", "document_type": "cnpj"}}"#,
                    CNPJ_VALIDO_SEM_MASCARA
                );
                let result: Result<Document, _> = serde_json::from_str(&json);
                assert!(result.is_ok());
            }

            #[test]
            fn should_fail_to_deserialize_when_cpf_number_is_invalid() {
                let json = r#"{"number": "00000000000", "document_type": "cpf"}"#;
                let result: Result<Document, _> = serde_json::from_str(json);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_to_deserialize_when_document_type_is_unknown() {
                let json = format!(
                    r#"{{"number": "{}", "document_type": "rg"}}"#,
                    CPF_VALIDO_SEM_MASCARA
                );
                let result: Result<Document, _> = serde_json::from_str(&json);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_to_deserialize_when_number_field_is_missing() {
                let json = r#"{"document_type": "cpf"}"#;
                let result: Result<Document, _> = serde_json::from_str(json);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_to_deserialize_when_document_type_field_is_missing() {
                let json = format!(r#"{{"number": "{}"}}"#, CPF_VALIDO_SEM_MASCARA);
                let result: Result<Document, _> = serde_json::from_str(&json);
                assert!(result.is_err());
            }
        }

        mod edge_cases {
            use super::*;

            #[test]
            fn should_fail_when_cpf_is_empty_string() {
                let result = Document::new("", DocumentType::Cpf);
                assert!(result.is_err());
            }

            #[test]
            fn should_fail_when_cnpj_is_empty_string() {
                let result = Document::new("", DocumentType::Cnpj);
                assert!(result.is_err());
            }

            #[test]
            fn should_accept_cpf_with_only_digits_no_mask() {
                let result = Document::new(CPF_VALIDO_SEM_MASCARA, DocumentType::Cpf);
                assert!(result.is_ok());
            }

            #[test]
            fn should_accept_cnpj_with_only_digits_no_mask() {
                let result = Document::new(CNPJ_VALIDO_SEM_MASCARA, DocumentType::Cnpj);
                assert!(result.is_ok());
            }

            #[test]
            fn should_preserve_number_after_roundtrip_serialization() {
                let original = Document::new(CPF_VALIDO, DocumentType::Cpf).unwrap();
                let json = serde_json::to_string(&original).unwrap();
                let restored: Document = serde_json::from_str(&json).unwrap();
                assert_eq!(original.number(), restored.number());
            }

            #[test]
            fn should_preserve_document_type_after_roundtrip_serialization() {
                let original = Document::new(CNPJ_VALIDO, DocumentType::Cnpj).unwrap();
                let json = serde_json::to_string(&original).unwrap();
                let restored: Document = serde_json::from_str(&json).unwrap();
                assert_eq!(original.document_type(), restored.document_type());
            }

            #[test]
            fn should_fail_when_cpf_contains_only_letters() {
                let result = Document::new("abcdefghijk", DocumentType::Cpf);
                assert!(result.is_err());
            }
        }
    }
}