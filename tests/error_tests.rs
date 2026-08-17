use ap_dl::error::AppError;

#[test]
fn test_error_categorization() {
    assert_eq!(AppError::from_err_string("Connection reset by peer"), AppError::Network);
    assert_eq!(AppError::from_err_string("DNS lookup failed: reqwest error"), AppError::Network);
    assert_eq!(AppError::from_err_string("Operation timed out"), AppError::Network);
    assert_eq!(AppError::from_err_string("HTTP 404 Not Found"), AppError::NotFound);
    assert_eq!(AppError::from_err_string("video not found on server"), AppError::NotFound);
    assert_eq!(AppError::from_err_string("No space left on device"), AppError::Disk);
    assert_eq!(AppError::from_err_string("Permission denied on write"), AppError::Disk);
    assert_eq!(AppError::from_err_string("Invalid URL format"), AppError::InvalidUrl);
    assert_eq!(AppError::from_err_string("invalid pattern"), AppError::InvalidUrl);
    assert_eq!(AppError::from_err_string("Download paused"), AppError::Cancelled);
    assert_eq!(AppError::from_err_string("Download cancelled by user"), AppError::Cancelled);
    assert_eq!(AppError::from_err_string("Random unexpected failure"), AppError::StreamUnavailable);
}

#[test]
fn test_ui_summaries() {
    assert_eq!(AppError::Network.to_ui_summary(), "Network Error");
    assert_eq!(AppError::NotFound.to_ui_summary(), "Video Not Found (404)");
    assert_eq!(AppError::Disk.to_ui_summary(), "Disk Error");
    assert_eq!(AppError::InvalidUrl.to_ui_summary(), "Invalid Aparat Link");
    assert_eq!(AppError::StreamUnavailable.to_ui_summary(), "Stream Unavailable");
    assert_eq!(AppError::Cancelled.to_ui_summary(), "Paused");
}
