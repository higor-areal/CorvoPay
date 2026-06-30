use serde::Serialize;

// ── Enums compartilhados ───────────────────────────────
#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MpPaymentMethodId {
    Pix,
    Master,
    Visa,
    Elo,
    Hipercard,
    Amex,
    Boleto,
    Pec,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MpPaymentMethodType {
    BankTransfer,
    CreditCard,
    DebitCard,
    Ticket,
}

// ── Payer ──────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct Identification {
    #[serde(rename = "type")]
    pub id_type: String,
    pub number: String,
}

#[derive(Debug, Serialize)]
pub struct Address {
    pub street_name: String,
    pub street_number: String,
    pub zip_code: String,
    pub neighborhood: String,
    pub state: String,
    pub city: String,
}

#[derive(Debug, Serialize)]
pub struct Payer {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<Identification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

// ── Payment methods ────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PixPaymentMethod {
    pub id: MpPaymentMethodId,
    #[serde(rename = "type")]
    pub type_payment: MpPaymentMethodType,
}

#[derive(Debug, Serialize)]
pub struct CardPaymentMethod {
    pub id: MpPaymentMethodId,
    #[serde(rename = "type")]
    pub type_payment: MpPaymentMethodType,
    pub token: String,
    pub installments: u8,
}

#[derive(Debug, Serialize)]
pub struct BoletoPaymentMethod {
    pub id: MpPaymentMethodId,
    #[serde(rename = "type")]
    pub type_payment: MpPaymentMethodType,
}

// ── Payment ────────────────────────────────────────────
#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Payment {
    Pix {
        amount: String,
        payment_method: PixPaymentMethod,
        #[serde(skip_serializing_if = "Option::is_none")]
        expiration_time: Option<String>,
    },
    Card {
        amount: String,
        payment_method: CardPaymentMethod,
    },
    Boleto {
        amount: String,
        payment_method: BoletoPaymentMethod,
    },
}

// ── Transactions ───────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct Transactions {
    pub payments: Vec<Payment>,
}

// ── Order ──────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct CreateOrderRequest {
    #[serde(rename = "type")]
    pub order_type: String,
    pub processing_mode: String,
    pub total_amount: String,
    pub external_reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub payer: Payer,
    pub transactions: Transactions,
}