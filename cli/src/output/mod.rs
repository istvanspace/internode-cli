use serde::Serialize;
use serde_json::Value;

use crate::error::CliError;

#[derive(Serialize)]
pub struct SuccessEnvelope {
    pub ok: bool,
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Serialize)]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct ErrorEnvelope {
    pub ok: bool,
    pub error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

pub fn print_success(data: Value) {
    let envelope = SuccessEnvelope {
        ok: true,
        data,
        meta: None,
    };
    println!("{}", serde_json::to_string(&envelope).unwrap());
}

pub fn print_error(err: &CliError) {
    let envelope = ErrorEnvelope {
        ok: false,
        error: ErrorDetail {
            code: err.error_code().to_string(),
            message: err.to_string(),
        },
    };
    println!("{}", serde_json::to_string(&envelope).unwrap());
}

pub fn stderr_msg(msg: &str) {
    eprintln!("internode: {msg}");
}
