use serde::{Serialize, Deserialize};

use crate::domain::value_objects::gateway::Gateway;
/// Status normalizado — traduzido pelo mapper de cada gateway.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,        // MP: action_required / PM: waiting_payment
    Paid,           // MP: approved      / PMpaid: 
    Failed,         // MP: rejected      / PM: failed | with_error
    Refunded,       // MP: refunded      / PM: refunded
    PendingRefund,  // PM: pending_refund (MP não tem equivalente direto)
    Expired,        // Pix vencido sem pagamento
    InvalidResponse // Se a resposta não é mapeada
}

#[allow(dead_code)]
impl PaymentStatus{
    pub fn from_gateway(gateway: Gateway, status: &str) -> Self{
        match gateway {
            Gateway::MercadoPago =>Self::from_mercado_pago(status),
            Gateway::Pagarme => Self::from_pagarme(status),
            Gateway::Stripe => Self::from_stripe(status)
        }
    }
}

#[allow(dead_code)]
impl PaymentStatus {
    fn from_mercado_pago(status: &str) -> Self {
        match status {
            "approved" => Self::Paid,

            "action_required" => Self::Pending,
            "pending" => Self::Pending,
            "in_process" => Self::Pending,

            "rejected" => Self::Failed,
            "cancelled" => Self::Failed,
            "charged_back" => Self::Failed,

            "refunded" => Self::Refunded,

            "expired" => Self::Expired,

            _ => Self::InvalidResponse,
        }
    }

    fn from_pagarme(status: &str) -> Self {
        todo!("implement pagarme status mapping {status}");
    }

    fn from_stripe(status: &str) -> Self {
        todo!("implement pagarme status mapping{status}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod domain {
        use super::*;

        // --- Paid ---

        #[test]
        fn should_return_paid_when_mercado_pago_status_is_approved() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "approved");
            assert_eq!(result, PaymentStatus::Paid);
        }

        // --- Pending ---

        #[test]
        fn should_return_pending_when_mercado_pago_status_is_action_required() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "action_required");
            assert_eq!(result, PaymentStatus::Pending);
        }

        #[test]
        fn should_return_pending_when_mercado_pago_status_is_pending() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "pending");
            assert_eq!(result, PaymentStatus::Pending);
        }

        #[test]
        fn should_return_pending_when_mercado_pago_status_is_in_process() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "in_process");
            assert_eq!(result, PaymentStatus::Pending);
        }

        // --- Failed ---

        #[test]
        fn should_return_failed_when_mercado_pago_status_is_rejected() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "rejected");
            assert_eq!(result, PaymentStatus::Failed);
        }

        #[test]
        fn should_return_failed_when_mercado_pago_status_is_cancelled() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "cancelled");
            assert_eq!(result, PaymentStatus::Failed);
        }

        #[test]
        fn should_return_failed_when_mercado_pago_status_is_charged_back() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "charged_back");
            assert_eq!(result, PaymentStatus::Failed);
        }

        // --- Refunded ---

        #[test]
        fn should_return_refunded_when_mercado_pago_status_is_refunded() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "refunded");
            assert_eq!(result, PaymentStatus::Refunded);
        }

        // --- Expired ---

        #[test]
        fn should_return_expired_when_mercado_pago_status_is_expired() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "expired");
            assert_eq!(result, PaymentStatus::Expired);
        }

        // --- InvalidResponse ---

        #[test]
        fn should_return_invalid_response_when_mercado_pago_status_is_unknown() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "unknown_status");
            assert_eq!(result, PaymentStatus::InvalidResponse);
        }

        #[test]
        fn should_return_invalid_response_when_mercado_pago_status_is_empty_string() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "");
            assert_eq!(result, PaymentStatus::InvalidResponse);
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn should_return_invalid_response_when_status_is_uppercase_approved() {
            // from_mercado_pago faz match exato — case sensitive
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "APPROVED");
            assert_eq!(result, PaymentStatus::InvalidResponse);
        }

        #[test]
        fn should_return_invalid_response_when_status_has_leading_whitespace() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, " approved");
            assert_eq!(result, PaymentStatus::InvalidResponse);
        }

        #[test]
        fn should_return_invalid_response_when_status_has_trailing_whitespace() {
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "approved ");
            assert_eq!(result, PaymentStatus::InvalidResponse);
        }

        #[test]
        fn should_return_invalid_response_for_pagarme_only_status_pending_refund() {
            // pending_refund não existe no mapeamento do MercadoPago
            let result = PaymentStatus::from_gateway(Gateway::MercadoPago, "pending_refund");
            assert_eq!(result, PaymentStatus::InvalidResponse);
        }
    }
}