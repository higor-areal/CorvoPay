use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PixData{
    pub expires_in_seconds: u64,
}
