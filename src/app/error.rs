use crate::{
    domain::error::ValidationError, 
    gateways::error::GatewayError
};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("bad request: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("gateway error: {0}")]
    Gateway(#[from] GatewayError)
}