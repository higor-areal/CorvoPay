use crate::domain::value_objects::payment_method_response::{boleto::BoletoData, card::CardData, pix::PixData};

use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentMethodResponse{
    Pix(PixData),
    Card(CardData),
    Boleto(BoletoData)
}