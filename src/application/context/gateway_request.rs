use crate::{application::dto::create_payment_intent::PaymentIntentInput, domain::value_objects::gateway_attempt_id::GatewayAttemptId};

#[allow(dead_code)]
pub struct GatewayRequestContext<'a> {
    pub payment_intent: &'a PaymentIntentInput,
    pub gateway_attempt_id: GatewayAttemptId,
}