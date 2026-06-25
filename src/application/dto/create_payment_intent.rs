use serde::Deserialize;

use crate::domain::value_objects::{
    amount::Amount,
    customer::payer::Payer,
    payment_method::payment_method::PaymentMethod,
};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct PaymentIntentInput{
    pub amount: Amount,
    pub payer: Payer,
    pub payment_method: PaymentMethod,
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::domain::value_objects::{
        amount::Amount,
        customer::payer::Payer,
        payment_method::{
            payment_method::PaymentMethod,
            pix::PixData,
            card::CardData,
            boleto::BoletoData,
        },
    };
    use crate::application::dto::create_payment_intent::PaymentIntentInput;

    // --- constantes ---
    const EMAIL_VALIDO: &str = "joao@exemplo.com";
    const NOME_VALIDO: &str = "João Silva";
    const CPF_VALIDO: &str = "52998224725";
    const CNPJ_VALIDO: &str = "11222333000181";
    const AMOUNT_VALIDO: u64 = 5_000;

    // --- helpers ---
    fn payer_cpf() -> Payer {
        use crate::domain::value_objects::customer::document::DocumentType;
        Payer::new(EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO, DocumentType::Cpf).unwrap()
    }

    fn payer_cnpj() -> Payer {
        use crate::domain::value_objects::customer::document::DocumentType;
        Payer::new(EMAIL_VALIDO, NOME_VALIDO, CNPJ_VALIDO, DocumentType::Cnpj).unwrap()
    }

    fn amount() -> Amount {
        Amount::new(AMOUNT_VALIDO).unwrap()
    }

    fn pix() -> PaymentMethod {
        PaymentMethod::Pix(PixData { expires_in_seconds: 3600 })
    }

    fn card() -> PaymentMethod {
        PaymentMethod::Card(CardData { numer: "4111111111111111".to_string() })
    }

    fn boleto() -> PaymentMethod {
        PaymentMethod::Boleto(BoletoData { expires_in_dasy: 3 })
    }

    fn json_payer_cpf() -> String {
        format!(
            r#"{{
                "email":"{}",
                "name":"{}",
                "document":{{
                    "number":"{}",
                    "document_type":"cpf"
                }}}}"#,
            EMAIL_VALIDO, NOME_VALIDO, CPF_VALIDO
        )
    }

    fn json_payer_cnpj() -> String {
        format!(
            r#"{{"email":"{}","name":"{}","document":{{"number":"{}","document_type":"cnpj"}}}}"#,
            EMAIL_VALIDO, NOME_VALIDO, CNPJ_VALIDO
        )
    }

    // -------------------------------------------------------------------------
    mod deserialize {
        use super::*;

        #[test]
        fn should_deserialize_payment_intent_with_pix() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_payment_intent_with_card() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"card","numer":"4111111111111111"}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_payment_intent_with_boleto() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"boleto","expires_in_dasy":3}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_payment_intent_with_cnpj_payer() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cnpj()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_amount_correctly() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let input: PaymentIntentInput = serde_json::from_str(&json).unwrap();
            assert_eq!(input.amount.cents(), AMOUNT_VALIDO);
        }

        #[test]
        fn should_deserialize_payer_email_correctly() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let input: PaymentIntentInput = serde_json::from_str(&json).unwrap();
            assert_eq!(input.payer.email.as_str(), EMAIL_VALIDO);
        }

        #[test]
        fn should_deserialize_pix_expires_in_seconds_correctly() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":7200}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let input: PaymentIntentInput = serde_json::from_str(&json).unwrap();
            match input.payment_method {
                PaymentMethod::Pix(data) => assert_eq!(data.expires_in_seconds, 7200),
                _ => panic!("expected Pix variant"),
            }
        }

        #[test]
        fn should_deserialize_boleto_expires_in_days_correctly() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"boleto","expires_in_dasy":5}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let input: PaymentIntentInput = serde_json::from_str(&json).unwrap();
            match input.payment_method {
                PaymentMethod::Boleto(data) => assert_eq!(data.expires_in_dasy, 5),
                _ => panic!("expected Boleto variant"),
            }
        }

        #[test]
        fn should_fail_when_amount_is_zero() {
            let json = format!(
                r#"{{"amount":0,"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_amount_exceeds_maximum() {
            let json = format!(
                r#"{{"amount":100000001,"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_payer_email_is_invalid() {
            let json = format!(
                r#"{{"amount":{},"payer":{{"email":"nao-eh-email","name":"{}","document":{{"number":"{}","document_type":"cpf"}}}},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO, NOME_VALIDO, CPF_VALIDO
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_payer_document_is_invalid() {
            let json = format!(
                r#"{{"amount":{},"payer":{{"email":"{}","name":"{}","document":{{"number":"00000000000","document_type":"cpf"}}}},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO, EMAIL_VALIDO, NOME_VALIDO
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_payment_method_type_is_unknown() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"crypto"}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_amount_field_is_missing() {
            let json = format!(
                r#"{{"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_payer_field_is_missing() {
            let json = format!(
                r#"{{"amount":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_payment_method_field_is_missing() {
            let json = format!(
                r#"{{"amount":{},"payer":{}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_payment_method_type_tag_is_missing() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_err());
        }
    }

    // -------------------------------------------------------------------------
    mod edge_cases {
        use super::*;

        #[test]
        fn should_deserialize_with_minimum_valid_amount() {
            let json = format!(
                r#"{{"amount":1,"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_with_maximum_valid_amount() {
            let json = format!(
                r#"{{"amount":100000000,"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                json_payer_cpf()
            );
            let result: Result<PaymentIntentInput, _> = serde_json::from_str(&json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_match_pix_variant_after_deserialization() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let input: PaymentIntentInput = serde_json::from_str(&json).unwrap();
            assert!(matches!(input.payment_method, PaymentMethod::Pix(_)));
        }

        #[test]
        fn should_match_card_variant_after_deserialization() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"card","numer":"4111111111111111"}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let input: PaymentIntentInput = serde_json::from_str(&json).unwrap();
            assert!(matches!(input.payment_method, PaymentMethod::Card(_)));
        }

        #[test]
        fn should_match_boleto_variant_after_deserialization() {
            let json = format!(
                r#"{{"amount":{},"payer":{},"payment_method":{{"type":"boleto","expires_in_dasy":3}}}}"#,
                AMOUNT_VALIDO,
                json_payer_cpf()
            );
            let input: PaymentIntentInput = serde_json::from_str(&json).unwrap();
            assert!(matches!(input.payment_method, PaymentMethod::Boleto(_)));
        }

        #[test]
        fn should_normalize_payer_email_to_lowercase_during_deserialization() {
            let json = format!(
                r#"{{"amount":{},"payer":{{"email":"JOAO@EXEMPLO.COM","name":"{}","document":{{"number":"{}","document_type":"cpf"}}}},"payment_method":{{"type":"pix","expires_in_seconds":3600}}}}"#,
                AMOUNT_VALIDO, NOME_VALIDO, CPF_VALIDO
            );
            let input: PaymentIntentInput = serde_json::from_str(&json).unwrap();
            assert_eq!(input.payer.email.as_str(), "joao@exemplo.com");
        }
    }
}