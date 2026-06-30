use crate::{
    contracts::gateway::Gateway, domain::gateway::gateway_info::GatewayInfo, 
    gateways::mercado_pago::{
        client::MercadoPagoClient, info::INFO_MERCADO_PAGO
    }
};

#[allow(dead_code)]
pub struct MercadoPago {
    client: MercadoPagoClient
}

impl Gateway for MercadoPago{
    fn info(&self) -> &'static GatewayInfo {
        &INFO_MERCADO_PAGO
    }
}
