use std::fmt;
use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success = 0,
    BadInput = 1,
    AuthError = 2,
    ServerError = 3,
    NetworkError = 4,
}

impl From<ExitCode> for process::ExitCode {
    fn from(code: ExitCode) -> Self {
        process::ExitCode::from(code as u8)
    }
}

#[derive(Debug)]
pub enum CliError {
    BadInput(String),
    Auth(String),
    Server(String),
    Network(String),
    Internal(String),
}

impl CliError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::BadInput(_) => ExitCode::BadInput,
            Self::Auth(_) => ExitCode::AuthError,
            Self::Server(_) => ExitCode::ServerError,
            Self::Network(_) => ExitCode::NetworkError,
            Self::Internal(_) => ExitCode::ServerError,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::BadInput(_) => "BAD_INPUT",
            Self::Auth(_) => "AUTH_ERROR",
            Self::Server(_) => "SERVER_ERROR",
            Self::Network(_) => "NETWORK_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadInput(msg) => write!(f, "{msg}"),
            Self::Auth(msg) => write!(f, "{msg}"),
            Self::Server(msg) => write!(f, "{msg}"),
            Self::Network(msg) => write!(f, "{msg}"),
            Self::Internal(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() || err.is_connect() {
            Self::Network(err.to_string())
        } else if err.is_status() {
            let status = err.status().unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
            if status == reqwest::StatusCode::UNAUTHORIZED
                || status == reqwest::StatusCode::FORBIDDEN
            {
                Self::Auth(format!("HTTP {status}"))
            } else if status.is_client_error() {
                Self::BadInput(format!("HTTP {status}: {err}"))
            } else {
                Self::Server(format!("HTTP {status}: {err}"))
            }
        } else {
            Self::Network(err.to_string())
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        Self::Internal(format!("JSON error: {err}"))
    }
}
