use crate::domain::gateway::{gateway::GatewayName, gateway_capabilities::GatewayCapabilities, gateway_info::GatewayInfo};


#[allow(dead_code)]
pub const INFO_MERCADO_PAGO: GatewayInfo = GatewayInfo {
    name: GatewayName::MercadoPago,

    capabilities: GatewayCapabilities {
        pix: false,
        card: false,
        boleto: false,
    },

    base_url: "https://api.mercadopago.com",
    api_version: "v1"
};