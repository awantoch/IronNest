use thiserror::Error;

#[derive(Error, Debug)]
pub enum TpLinkError {
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Device discovery timeout")]
    DiscoveryTimeout,

    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),

    #[error("Invalid color format: {0}")]
    InvalidColor(String),

    #[error("Device communication error: {message}")]
    DeviceCommunication { message: String },

    #[error("Unsupported device type: {device_type}")]
    UnsupportedDevice { device_type: String },
}

pub type Result<T> = std::result::Result<T, TpLinkError>;
