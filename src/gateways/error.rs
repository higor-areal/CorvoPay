use crate::domain::gateway::gateway::GatewayName;
use reqwest;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum GatewayError {
    // ── Mapper / Integração ───────────────────────────────────────────────

    #[error("invalid payment method for this gateway operation")]
    InvalidPaymentMethod,

    #[error("mapper error: {reason}")]
    Mapper {
        reason: String,
    },

    // ── HTTP ──────────────────────────────────────────────────────────────

    #[error("[{gateway:?}] unauthorized")]
    Unauthorized {
        gateway: GatewayName,
    },

    #[error("[{gateway:?}] network error: {source}")]
    Network {
        gateway: GatewayName,
        source: reqwest::Error,
    },

    #[error("[{gateway:?}] invalid response: {reason}")]
    InvalidResponse {
        gateway: GatewayName,
        reason: String,
    },

    #[error("[{gateway:?}] api error ({status}): {message}")]
    ApiError {
        gateway: GatewayName,
        status: u16,
        message: String,
    },
}

#[allow(dead_code)]
pub trait GatewayResultExt<T> {
    fn gateway_err(self, gateway: GatewayName) -> Result<T, GatewayError>;
}

impl<T> GatewayResultExt<T> for Result<T, reqwest::Error> {
    fn gateway_err(self, gateway: GatewayName) -> Result<T, GatewayError> {
        self.map_err(|e| GatewayError::Network {
            gateway,
            source: e,
        })
    }
}