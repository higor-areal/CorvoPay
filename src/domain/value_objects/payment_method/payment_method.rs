use serde::{Serialize, Deserialize};
use crate::domain::value_objects::payment_method::{
    pix::PixData,
    card::CardData,
    boleto::BoletoData
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentMethod{
    Pix(PixData),
    Card(CardData),
    Boleto(BoletoData),
}
