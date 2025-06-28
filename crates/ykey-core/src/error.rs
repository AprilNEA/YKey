// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

//! Error types and result handling for YKey

use thiserror::Error;

/// Result type used throughout the YKey ecosystem
pub type YKeyResult<T> = Result<T, YKeyError>;

/// Comprehensive error types for YKey operations
#[derive(Debug, Error)]
pub enum YKeyError {
    /// Device not found with the given identifier
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// Unsupported device type
    #[error("Unsupported device type: {0:?}")]
    UnsupportedDevice(crate::types::DeviceType),

    /// Device communication error
    #[error("Device communication error: {0}")]
    CommunicationError(String),

    /// CTAP (Client to Authenticator Protocol) error
    #[error("CTAP error code: {code:#04x} - {message}")]
    CtapError { code: u8, message: String },

    /// Unexpected response from device
    #[error("Unexpected response from device")]
    UnexpectedResponse,

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Operation cancelled by user
    #[error("Operation cancelled by user")]
    UserCancelled,

    /// Invalid PIN provided
    #[error("Invalid PIN: {0}")]
    InvalidPin(String),

    /// Device is locked (too many PIN attempts)
    #[error("Device is locked")]
    DeviceLocked,

    /// Device requires PIN verification
    #[error("PIN verification required")]
    PinRequired,

    /// Device requires user verification (touch/biometric)
    #[error("User verification required")]
    UserVerificationRequired,

    /// Credential not found
    #[error("Credential not found: {0}")]
    CredentialNotFound(String),

    /// Invalid credential data
    #[error("Invalid credential data: {0}")]
    InvalidCredential(String),

    /// Protocol version not supported
    #[error("Unsupported protocol version: {0}")]
    UnsupportedProtocolVersion(String),

    /// Invalid request parameters
    #[error("Invalid request parameters: {0}")]
    InvalidParameters(String),

    /// Timeout occurred during operation
    #[error("Operation timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    /// Permission denied (typically OS-level permissions)
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Device already in use by another process
    #[error("Device busy: {0}")]
    DeviceBusy(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Generic error with context
    #[error("Operation failed: {0}")]
    Generic(#[from] anyhow::Error),
}

impl YKeyError {
    /// Create a new communication error
    pub fn communication<S: Into<String>>(message: S) -> Self {
        Self::CommunicationError(message.into())
    }

    /// Create a new CTAP error with code and message
    pub fn ctap_error(code: u8) -> Self {
        let message = match code {
            0x01 => "Invalid command".to_string(),
            0x02 => "Invalid parameter".to_string(),
            0x03 => "Invalid length".to_string(),
            0x04 => "Invalid sequence".to_string(),
            0x05 => "Message timeout".to_string(),
            0x06 => "Channel busy".to_string(),
            0x0A => "Lock required".to_string(),
            0x0B => "Invalid channel".to_string(),
            0x10 => "CBOR unexpected type".to_string(),
            0x11 => "Invalid CBOR".to_string(),
            0x12 => "Missing parameter".to_string(),
            0x13 => "Limit exceeded".to_string(),
            0x14 => "Unsupported extension".to_string(),
            0x15 => "Credential excluded".to_string(),
            0x16 => "Processing".to_string(),
            0x17 => "Invalid credential".to_string(),
            0x18 => "User action pending".to_string(),
            0x19 => "Operation pending".to_string(),
            0x1A => "No operations".to_string(),
            0x1B => "Unsupported algorithm".to_string(),
            0x1C => "Operation denied".to_string(),
            0x1D => "Key store full".to_string(),
            0x1E => "No operation pending".to_string(),
            0x1F => "Unsupported option".to_string(),
            0x20 => "Invalid option".to_string(),
            0x21 => "Keep alive cancel".to_string(),
            0x22 => "No credentials".to_string(),
            0x23 => "User action timeout".to_string(),
            0x24 => "Not allowed".to_string(),
            0x25 => "PIN invalid".to_string(),
            0x26 => "PIN blocked".to_string(),
            0x27 => "PIN auth invalid".to_string(),
            0x28 => "PIN auth blocked".to_string(),
            0x29 => "PIN not set".to_string(),
            0x2A => "PIN required".to_string(),
            0x2B => "PIN policy violation".to_string(),
            0x2C => "PIN token expired".to_string(),
            0x2D => "Request too large".to_string(),
            0x2E => "Action timeout".to_string(),
            0x2F => "Up required".to_string(),
            0x30 => "UV blocked".to_string(),
            0x31 => "Integrity failure".to_string(),
            0x32 => "Invalid subcommand".to_string(),
            0x33 => "UV invalid".to_string(),
            0x34 => "Unauthorized permission".to_string(),
            _ => format!("Unknown error code: {:#04x}", code),
        };
        
        Self::CtapError { code, message }
    }

    /// Create a timeout error
    pub fn timeout(seconds: u64) -> Self {
        Self::Timeout { seconds }
    }

    /// Create an authentication failed error
    pub fn auth_failed<S: Into<String>>(reason: S) -> Self {
        Self::AuthenticationFailed(reason.into())
    }

    /// Create a permission denied error
    pub fn permission_denied<S: Into<String>>(resource: S) -> Self {
        Self::PermissionDenied(resource.into())
    }

    /// Check if this error indicates the device is locked
    pub fn is_device_locked(&self) -> bool {
        matches!(
            self,
            YKeyError::DeviceLocked | YKeyError::CtapError { code: 0x26, .. }
        )
    }

    /// Check if this error indicates PIN is required
    pub fn is_pin_required(&self) -> bool {
        matches!(
            self,
            YKeyError::PinRequired | YKeyError::CtapError { code: 0x2A, .. }
        )
    }

    /// Check if this error indicates user verification is required
    pub fn is_user_verification_required(&self) -> bool {
        matches!(
            self,
            YKeyError::UserVerificationRequired | YKeyError::CtapError { code: 0x2F, .. }
        )
    }

    /// Check if this is a temporary error that might succeed on retry
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            YKeyError::DeviceBusy(_)
                | YKeyError::Timeout { .. }
                | YKeyError::CommunicationError(_)
                | YKeyError::CtapError { code: 0x06, .. } // Channel busy
                | YKeyError::CtapError { code: 0x16, .. } // Processing
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctap_error_messages() {
        let error = YKeyError::ctap_error(0x25);
        assert!(error.to_string().contains("PIN invalid"));
        
        let error = YKeyError::ctap_error(0x26);
        assert!(error.to_string().contains("PIN blocked"));
        assert!(error.is_device_locked());
    }

    #[test]
    fn test_error_classification() {
        let pin_error = YKeyError::ctap_error(0x2A);
        assert!(pin_error.is_pin_required());
        assert!(!pin_error.is_device_locked());

        let busy_error = YKeyError::DeviceBusy("test".to_string());
        assert!(busy_error.is_retryable());
        assert!(!busy_error.is_pin_required());
    }

    #[test]
    fn test_timeout_error() {
        let timeout = YKeyError::timeout(30);
        assert!(timeout.is_retryable());
        assert!(timeout.to_string().contains("30 seconds"));
    }

    #[test]
    fn test_communication_error() {
        let comm_error = YKeyError::communication("Failed to send data");
        assert!(comm_error.is_retryable());
        assert!(comm_error.to_string().contains("Failed to send data"));
    }
} 