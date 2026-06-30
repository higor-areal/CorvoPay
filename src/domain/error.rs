use serde::Serialize;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error, PartialEq, Serialize)]
pub enum ValidationError {
    #[error("amount must be greater than zero")]
    InvalidAmount,
    #[error("currency is required")]
    InvalidCurrency,
    #[error("payer email is required")]
    InvalidEmail,
    #[error("payer name is required")]
    InvalidName,
    #[error("payer document is required")]
    InvalidDocument,
    #[error("idempotency key is required")]
    InvalidIdempotencyKey,
    #[error("Qr code invalid")]
    PixQrCodeError
}