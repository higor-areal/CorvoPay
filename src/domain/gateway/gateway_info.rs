use crate::domain::gateway::{
    gateway::GatewayName, gateway_capabilities::GatewayCapabilities
};

//talvez precise mudar alguma coisa aqui posteriormente
#[allow(dead_code)]
pub struct GatewayInfo {
    pub name: GatewayName,
    pub capabilities: GatewayCapabilities,
    pub base_url: &'static str,
    pub api_version: &'static str,
}