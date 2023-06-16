use std::io;

use serde_json::Value;

use crate::mc::mctypes::MCString;

pub mod status_response;
pub mod login_success;
pub mod ping_response;

#[derive(Debug)]
#[allow(unused)]
pub struct JsonResponse {
    data: Value
}

impl JsonResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let mc_string = MCString::from_bytes(bytes)?;

        let value: Value = serde_json::from_str(&mc_string.string())?;

        Ok(JsonResponse { data: value })
    }
}