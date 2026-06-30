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
