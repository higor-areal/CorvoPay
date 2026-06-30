use serde::{Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug)]
pub struct BoletoData {
    pub barcode: String,
    pub digitable_line: String,
    pub pdf_url: Option<String>,
    pub expires_at: DateTime<Utc>,
}