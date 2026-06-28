use crate::domain::value_objects::{
    amount::Amount, payment_method_response::payment_method_response::PaymentMethodResponse, payment_status::PaymentStatus
};

#[allow(dead_code)]
pub struct PaymentIntentOutput{
    pub id: String,
    pub status: PaymentStatus,
    pub amount: Amount,
    pub payment_response: PaymentMethodResponse
}