#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AppError {
    Network,
    NotFound,
    Disk,
    InvalidUrl,
    StreamUnavailable,
    Cancelled,
}

impl AppError {
    pub fn to_ui_summary(&self) -> String {
        match self {
            AppError::Network => "Network Error".to_string(),
            AppError::NotFound => "Video Not Found (404)".to_string(),
            AppError::Disk => "Disk Error".to_string(),
            AppError::InvalidUrl => "Invalid Aparat Link".to_string(),
            AppError::StreamUnavailable => "Stream Unavailable".to_string(),
            AppError::Cancelled => "Paused".to_string(),
        }
    }

    pub fn from_err_string(err: &str) -> Self {
        let lower = err.to_lowercase();
        if lower.contains("paused") || lower.contains("cancel") {
            AppError::Cancelled
        } else if lower.contains("404") || lower.contains("not found") {
            AppError::NotFound
        } else if lower.contains("network")
            || lower.contains("connection")
            || lower.contains("timeout")
            || lower.contains("timed out")
            || lower.contains("dns")
            || lower.contains("reqwest")
        {
            AppError::Network
        } else if lower.contains("disk")
            || lower.contains("write")
            || lower.contains("permission")
            || lower.contains("space")
        {
            AppError::Disk
        } else if lower.contains("invalid") || lower.contains("url") || lower.contains("pattern") {
            AppError::InvalidUrl
        } else {
            AppError::StreamUnavailable
        }
    }
}
