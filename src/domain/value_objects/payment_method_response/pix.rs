use serde::{Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug)]
pub struct PixData {
    pub qr_code: String,
    pub qr_code_base64: Option<String>,
    pub qr_code_url: Option<String>,
    pub expires_at: DateTime<Utc>,
}