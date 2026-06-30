use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoletoData{
    pub expires_in_dasy: u32,
}
