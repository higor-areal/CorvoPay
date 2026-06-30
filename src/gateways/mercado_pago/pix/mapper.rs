use crate::{
    application::context::gateway_request::GatewayRequestContext, domain::{
        gateway::gateway::GatewayName, value_objects::{customer::document, payment_method::payment_method::PaymentMethod}
    }, gateways::{
            error::GatewayError, mercado_pago::{request::{CreateOrderRequest, Identification, MpPaymentMethodId, MpPaymentMethodType, Payer, Payment, PixPaymentMethod, Transactions}, utils::seconds_to_iso8601}
        }
    };



#[allow(dead_code)]
const EXPIRATION_MIN_SECS: u32 = 1_800;     // 30 minutos
#[allow(dead_code)]
const EXPIRATION_MAX_SECS: u32 = 2_592_000; // 30 dias
#[allow(dead_code)]
const GATEWAY_NAME: GatewayName = GatewayName::MercadoPago;
#[allow(dead_code)]
const ORDER_TYPE: & 'static str = "online";
#[allow(dead_code)]
const PROCESSING_MODE: & 'static str = "automatic";

#[allow(dead_code)]
pub struct MpPixMapper;

#[allow(dead_code)]
impl MpPixMapper{

    pub fn to_request(
        input: &GatewayRequestContext<'_>
    ) -> Result<CreateOrderRequest, GatewayError>{

        // INTENT
        let intent = input.payment_intent;

        // ATTEMPT
        let id_attempt = &input.gateway_attempt_id;

        // PIX
        let PaymentMethod::Pix(pix) = &intent.payment_method
        else {
            return Err(GatewayError::InvalidPaymentMethod)
        };

        // AMOUNT
        let amount = intent.amount.cents().to_string();

        // EXPIRATION
        let expiration_time = 
            pix
            .expires_in_seconds
            .clamp(
                EXPIRATION_MIN_SECS, 
                EXPIRATION_MAX_SECS
        );

        // PAYER
        let payer = &intent.payer;

        let document_type = match payer.document.document_type() {
            document::DocumentType::Cpf => "CPF",
            document::DocumentType::Cnpj => "CNPJ"
        };

        let document = Identification{
            id_type: document_type.to_string(),
            number: payer.document.number().to_string()
        };

        let mp_payer = Payer{
            email: payer.email.as_str().to_string(),
            first_name: Some(payer.name.as_str().to_string()),
            last_name: None,
            identification: Some(document), 
            address: None
        };

        //PAYMENT METHOD
        let payment_method = PixPaymentMethod{
            id: MpPaymentMethodId::Pix,
            type_payment: MpPaymentMethodType::BankTransfer
        };

        //PAYMENT
        let payment =Payment::Pix { 
            amount: amount.clone(), 
            payment_method: payment_method,
            expiration_time: Some(seconds_to_iso8601(expiration_time)) 
        };

        //TRANSACTIONS
        let transactions = Transactions{
            payments: vec![payment]
        };


        Ok(
            CreateOrderRequest { 
                order_type: ORDER_TYPE.to_string(), 
                processing_mode: PROCESSING_MODE.to_string(), 
                total_amount: amount, 
                external_reference: id_attempt.as_uuid().to_string(), 
                description: None, 
                payer: mp_payer, 
                transactions: transactions
            }
        )
    }

//    pub fn to_response(response: )
}



#[cfg(test)]
mod tests {
    use crate::application::context::gateway_request::GatewayRequestContext;
    use crate::application::dto::create_payment_intent::PaymentIntentInput;
    use crate::domain::value_objects::amount::Amount;
    use crate::domain::value_objects::customer::payer::Payer;
    use crate::domain::value_objects::customer::document::DocumentType;
    use crate::domain::value_objects::gateway_attempt_id::GatewayAttemptId;
    use crate::domain::value_objects::payment_method::payment_method::PaymentMethod;
    use crate::domain::value_objects::payment_method::pix::PixData;
    use crate::gateways::error::GatewayError;
    use crate::gateways::mercado_pago::pix::mapper::MpPixMapper;
    use crate::gateways::mercado_pago::request::Payment;

    // --- DADOS FORNECIDOS ---
    const CPF: &str = "40134881087";
    const EMAIL: &str = "email@teste.com";
    const NOME: &str = "João Silva";
    const UUID_V7_VALIDO: &str = "01932b4a-f1c3-7a2e-9b1d-3f4e5a6b7c8d";

    // --- helpers ---
    fn payer() -> Payer {
        Payer::new(EMAIL, NOME, CPF, DocumentType::Cpf).unwrap()
    }

    fn amount() -> Amount {
        Amount::new(1000).unwrap() // 10,00
    }

    fn gateway_attempt_id() -> GatewayAttemptId {
        // SÓ PODE SER CRIADO NO DESERIALIZE
        serde_json::from_str(&format!("\"{}\"", UUID_V7_VALIDO)).unwrap()
    }

    fn pix_payment_intent() -> PaymentIntentInput {
        PaymentIntentInput {
            amount: amount(),
            payer: payer(),
            payment_method: PaymentMethod::Pix(PixData { expires_in_seconds: 3600 }),
        }
    }

    fn non_pix_payment_intent() -> PaymentIntentInput {
        use crate::domain::value_objects::payment_method::card::CardData;
        PaymentIntentInput {
            amount: amount(),
            payer: payer(),
            payment_method: PaymentMethod::Card(CardData {
                number: "4111111111111111".to_string(),
            }),
        }
    }

    // -------------------------------------------------------------------------
    mod domain {
        use super::*;

        #[test]
        fn should_build_request_successfully_for_pix_payment() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let result = MpPixMapper::to_request(&context);

            assert!(result.is_ok());
        }

        #[test]
        fn should_set_order_type_as_online() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert_eq!(request.order_type, "online");
        }

        #[test]
        fn should_set_processing_mode_as_automatic() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert_eq!(request.processing_mode, "automatic");
        }

        #[test]
        fn should_convert_amount_cents_to_string() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert_eq!(request.total_amount, "1000");
        }

        #[test]
        fn should_set_external_reference_as_gateway_attempt_uuid() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert_eq!(request.external_reference, UUID_V7_VALIDO);
        }

        #[test]
        fn should_set_description_as_none() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert!(request.description.is_none());
        }

        #[test]
        fn should_set_payer_email_correctly() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert_eq!(request.payer.email, EMAIL);
        }

        #[test]
        fn should_set_payer_first_name_from_payer_name() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert_eq!(request.payer.first_name, Some(NOME.to_string()));
        }

        #[test]
        fn should_set_payer_last_name_as_none() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert!(request.payer.last_name.is_none());
        }

        #[test]
        fn should_set_payer_address_as_none() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert!(request.payer.address.is_none());
        }

        #[test]
        fn should_set_identification_type_as_cpf_when_document_is_cpf() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();
            let identification = request.payer.identification.unwrap();

            assert_eq!(identification.id_type, "CPF");
        }

        #[test]
        fn should_set_identification_number_as_normalized_document() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();
            let identification = request.payer.identification.unwrap();

            assert_eq!(identification.number, CPF);
        }

        #[test]
        fn should_set_identification_type_as_cnpj_when_document_is_cnpj() {
            let cnpj_payer = Payer::new(EMAIL, NOME, "11222333000181", DocumentType::Cnpj).unwrap();
            let intent = PaymentIntentInput {
                amount: amount(),
                payer: cnpj_payer,
                payment_method: PaymentMethod::Pix(PixData { expires_in_seconds: 3600 }),
            };
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();
            let identification = request.payer.identification.unwrap();

            assert_eq!(identification.id_type, "CNPJ");
        }

        #[test]
        fn should_create_single_payment_in_transactions() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            assert_eq!(request.transactions.payments.len(), 1);
        }

        #[test]
        fn should_set_payment_amount_matching_total_amount() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            match &request.transactions.payments[0] {
                Payment::Pix { amount, .. } => assert_eq!(amount, "1000"),
                _ => panic!("expected Payment::Pix variant"),
            }
        }

        #[test]
        fn should_fail_when_payment_method_is_not_pix() {
            let intent = non_pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let result = MpPixMapper::to_request(&context);

            assert!(matches!(result.unwrap_err(), GatewayError::InvalidPaymentMethod));
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn should_clamp_expiration_to_minimum_when_below_30_minutes() {
            let intent = PaymentIntentInput {
                amount: amount(),
                payer: payer(),
                payment_method: PaymentMethod::Pix(PixData { expires_in_seconds: 60 }), // 1 min
            };
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            match &request.transactions.payments[0] {
                Payment::Pix { expiration_time, .. } => {
                    // 1800s clamped = 30 minutos = PT30M
                    assert_eq!(expiration_time.as_deref(), Some("PT30M"));
                }
                _ => panic!("expected Payment::Pix variant"),
            }
        }

        #[test]
        fn should_clamp_expiration_to_maximum_when_above_30_days() {
            let intent = PaymentIntentInput {
                amount: amount(),
                payer: payer(),
                payment_method: PaymentMethod::Pix(PixData { expires_in_seconds: 9_999_999 }),
            };
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            match &request.transactions.payments[0] {
                Payment::Pix { expiration_time, .. } => {
                    // 2_592_000s clamped = 30 dias = P30D
                    assert_eq!(expiration_time.as_deref(), Some("P30D"));
                }
                _ => panic!("expected Payment::Pix variant"),
            }
        }

        #[test]
        fn should_keep_expiration_unchanged_when_within_valid_range() {
            let intent = pix_payment_intent(); // 3600s = 1 hora
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();

            match &request.transactions.payments[0] {
                Payment::Pix { expiration_time, .. } => {
                    assert_eq!(expiration_time.as_deref(), Some("PT1H"));
                }
                _ => panic!("expected Payment::Pix variant"),
            }
        }

        #[test]
        fn should_serialize_request_to_valid_json() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();
            let json = serde_json::to_string(&request);

            assert!(json.is_ok());
        }

        #[test]
        fn should_omit_description_field_when_serialized() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();
            let json = serde_json::to_string(&request).unwrap();

            assert!(!json.contains("description"));
        }

        #[test]
        fn should_rename_id_type_field_to_type_when_serialized() {
            let intent = pix_payment_intent();
            let context = GatewayRequestContext {
                payment_intent: &intent,
                gateway_attempt_id: gateway_attempt_id(),
            };

            let request = MpPixMapper::to_request(&context).unwrap();
            let json = serde_json::to_value(&request).unwrap();

            assert_eq!(json["payer"]["identification"]["type"], "CPF");
        }
    }
}
