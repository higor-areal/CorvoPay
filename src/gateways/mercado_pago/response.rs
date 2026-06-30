use serde::{Deserialize, Serialize};




#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MpPixResponse {
    pub id: String,

    #[serde(rename = "type")]
    pub type_data: String,

    pub total_amount: String,
    pub external_reference: String,
    pub country_code: String,
    pub status: String,
    pub status_detail: String,
    pub capture_mode: String,
    pub processing_mode: String,
    pub marketplace: String,

    pub transactions: MpResponseTransactions,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MpResponseTransactions {
    pub payments: Vec::<MpResponsePayment>,
}


#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MpResponsePayment {
    pub id: String,
    pub reference_id: String,
    pub status: String,
    pub status_detail: String,
    pub amount: String,
    pub payment_method: MpResponsePaymentMethod,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MpResponsePaymentMethod {
    /// Para Pix: "pix"
    pub id: MpPaymentMethodId,

    /// Para Pix: "bank_transfer"
    #[serde(rename = "type")]
    pub type_payment: MpPaymentMethodType,

    /// URL com QR Code, Pix Copia e Cola e instruções de pagamento.
    pub ticket_url: String,

    /// Código alfanumérico para copiar e colar.
    pub qr_code: String,

    /// Representação em Base64 da imagem do QR Code.
    pub qr_code_base64: String,
}


//uma segurança contra idiotas, no pix, só podemos receber o method_id pix
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MpPaymentMethodId {
    Pix,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MpPaymentMethodType {
    BankTransfer,
}