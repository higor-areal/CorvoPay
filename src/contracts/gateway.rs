use crate::domain::gateway::gateway_info::GatewayInfo;


#[allow(dead_code)]
pub trait Gateway{
    fn info(&self) -> &'static GatewayInfo;
}