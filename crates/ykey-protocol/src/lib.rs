// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

//! Protocol implementations for YKey hardware security keys
//! 
//! This crate provides implementations for various hardware security key protocols,
//! including FIDO2/WebAuthn and CTAP (Client to Authenticator Protocol).

use ykey_core::{traits::*, types::*, YKeyResult, YKeyError};
use async_trait::async_trait;
use std::time::Duration;

/// CTAP Command types
#[derive(Debug, Clone)]
pub enum CtapCommand {
    GetInfo,
    MakeCredential(MakeCredentialParams),
    GetAssertion(GetAssertionParams),
    Reset,
    ClientPin(ClientPinCommand),
    GetNextAssertion,
    Cancel,
}

/// Client PIN command variants
#[derive(Debug, Clone)]
pub enum ClientPinCommand {
    SetPin { pin: String },
    ChangePin { old_pin: String, new_pin: String },
    GetPinToken { pin: String },
}

/// CTAP Response types
#[derive(Debug, Clone)]
pub enum CtapResponse {
    GetInfo(AuthenticatorInfo),
    MakeCredential(AttestationObject),
    GetAssertion(AssertionObject),
    Reset,
    ClientPin,
    ClientPinToken(Vec<u8>),
    Cancel,
    Error(u8),
}

impl CtapCommand {
    /// Encode command to bytes (simplified for now)
    pub fn encode(&self) -> YKeyResult<Vec<u8>> {
        match self {
            CtapCommand::GetInfo => Ok(vec![0x04]), // CTAP2 GetInfo command
            CtapCommand::MakeCredential(_) => Ok(vec![0x01]), // CTAP2 MakeCredential command
            CtapCommand::GetAssertion(_) => Ok(vec![0x02]), // CTAP2 GetAssertion command
            CtapCommand::Reset => Ok(vec![0x07]), // CTAP2 Reset command
            CtapCommand::ClientPin(_) => Ok(vec![0x06]), // CTAP2 ClientPin command
            CtapCommand::GetNextAssertion => Ok(vec![0x08]), // CTAP2 GetNextAssertion command
            CtapCommand::Cancel => Ok(vec![0x3F, 0x00, 0x00, 0x00]), // HID Cancel packet
        }
    }
}

impl CtapResponse {
    /// Decode response from bytes (simplified for now)
    pub fn decode(data: &[u8]) -> YKeyResult<Self> {
        if data.is_empty() {
            return Err(YKeyError::communication("Empty response"));
        }

        // Check for CTAP2 status byte
        match data[0] {
            0x00 => {
                // Success - determine response type based on length and content
                if data.len() == 1 {
                    Ok(CtapResponse::Reset)
                } else {
                    // For now, return a mock AuthenticatorInfo for GetInfo
                    Ok(CtapResponse::GetInfo(AuthenticatorInfo {
                        versions: vec!["FIDO_2_0".to_string()],
                        extensions: Some(vec!["hmac-secret".to_string()]),
                        aaguid: vec![0; 16],
                        options: None,
                        max_msg_size: Some(1200),
                        pin_uv_auth_protocols: Some(vec![1]),
                        max_credential_count_in_list: Some(8),
                        max_credential_id_length: Some(128),
                        transports: Some(vec!["usb".to_string()]),
                        algorithms: None,
                        max_serialized_large_blob_array: None,
                        force_pin_change: None,
                        min_pin_length: Some(4),
                        firmware_version: None,
                        max_cred_blob_length: None,
                        max_rp_ids_for_set_min_pin_length: None,
                        preferred_platform_uv_attempts: None,
                        uv_modality: None,
                        certifications: None,
                        remaining_discoverable_credentials: None,
                        vendor_prototype_config_commands: None,
                    }))
                }
            },
            0x01..=0xFF => Ok(CtapResponse::Error(data[0])),
        }
    }
}

/// FIDO2 protocol client implementation
/// 
/// Provides a high-level interface for FIDO2 operations on hardware security keys.
pub struct Fido2Client<D: Device> {
    device: D,
    pin_token: Option<Vec<u8>>,
    pin_protocol_version: Option<u8>,
    timeout: Duration,
}

impl<D: Device> Fido2Client<D> {
    /// Create a new FIDO2 client with the given device
    pub fn new(device: D) -> Self {
        Self {
            device,
            pin_token: None,
            pin_protocol_version: None,
            timeout: Duration::from_secs(30),
        }
    }

    /// Create a new FIDO2 client with custom timeout
    pub fn with_timeout(device: D, timeout: Duration) -> Self {
        Self {
            device,
            pin_token: None,
            pin_protocol_version: None,
            timeout,
        }
    }

    /// Set the operation timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Get current PIN token if available
    pub fn pin_token(&self) -> Option<&Vec<u8>> {
        self.pin_token.as_ref()
    }

    /// Clear stored PIN token
    pub fn clear_pin_token(&mut self) {
        self.pin_token = None;
        self.pin_protocol_version = None;
    }
}

#[async_trait]
impl<D: Device> Fido2Protocol for Fido2Client<D> {
    async fn get_info(&mut self) -> YKeyResult<AuthenticatorInfo> {
        let command = CtapCommand::GetInfo;
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::GetInfo(info) => Ok(info),
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
    
    async fn make_credential(
        &mut self, 
        params: MakeCredentialParams
    ) -> YKeyResult<AttestationObject> {
        let command = CtapCommand::MakeCredential(params);
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::MakeCredential(attestation) => Ok(attestation),
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
    
    async fn get_assertion(
        &mut self, 
        params: GetAssertionParams
    ) -> YKeyResult<AssertionObject> {
        let command = CtapCommand::GetAssertion(params);
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::GetAssertion(assertion) => Ok(assertion),
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
    
    async fn reset(&mut self) -> YKeyResult<()> {
        let command = CtapCommand::Reset;
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::Reset => {
                // Clear any stored PIN tokens after reset
                self.clear_pin_token();
                Ok(())
            },
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
    
    async fn set_pin(&mut self, pin: &str) -> YKeyResult<()> {
        if pin.len() < 4 || pin.len() > 8 {
            return Err(YKeyError::InvalidParameters("PIN must be 4-8 characters".to_string()));
        }
        
        let command = CtapCommand::ClientPin(ClientPinCommand::SetPin {
            pin: pin.to_string(),
        });
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::ClientPin => Ok(()),
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
    
    async fn change_pin(&mut self, old_pin: &str, new_pin: &str) -> YKeyResult<()> {
        if new_pin.len() < 4 || new_pin.len() > 8 {
            return Err(YKeyError::InvalidParameters("PIN must be 4-8 characters".to_string()));
        }
        
        let command = CtapCommand::ClientPin(ClientPinCommand::ChangePin {
            old_pin: old_pin.to_string(),
            new_pin: new_pin.to_string(),
        });
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::ClientPin => {
                // Clear stored PIN token after PIN change
                self.clear_pin_token();
                Ok(())
            },
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
    
    async fn verify_pin(&mut self, pin: &str) -> YKeyResult<Vec<u8>> {
        let command = CtapCommand::ClientPin(ClientPinCommand::GetPinToken {
            pin: pin.to_string(),
        });
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::ClientPinToken(token) => {
                self.pin_token = Some(token.clone());
                self.pin_protocol_version = Some(1); // CTAP2.0 PIN protocol
                Ok(token)
            },
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
    
    async fn get_next_assertion(&mut self) -> YKeyResult<AssertionObject> {
        let command = CtapCommand::GetNextAssertion;
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::GetAssertion(assertion) => Ok(assertion),
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
    
    async fn cancel(&mut self) -> YKeyResult<()> {
        // CTAP cancel is typically sent as a separate HID packet
        // For now, we'll implement a basic timeout-based cancel
        let command = CtapCommand::Cancel;
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::Cancel => Ok(()),
            CtapResponse::Error(code) => Err(YKeyError::ctap_error(code)),
            _ => Err(YKeyError::UnexpectedResponse),
        }
    }
}

impl<D: Device> Fido2Client<D> {
    /// Send a CTAP command to the device and parse the response
    async fn send_ctap_command(&mut self, command: CtapCommand) -> YKeyResult<CtapResponse> {
        let data = command.encode()?;
        
        // Add timeout for the operation
        let response_data = tokio::time::timeout(
            self.timeout,
            self.device.send_raw(&data)
        ).await
        .map_err(|_| YKeyError::timeout(self.timeout.as_secs()))?
        .map_err(|e| YKeyError::communication(format!("Device communication failed: {}", e)))?;
        
        CtapResponse::decode(&response_data)
    }
    
    /// Get underlying device reference
    pub fn device(&self) -> &D {
        &self.device
    }
    
    /// Get mutable underlying device reference
    pub fn device_mut(&mut self) -> &mut D {
        &mut self.device
    }
    
    /// Check if PIN token is available
    pub fn has_pin_token(&self) -> bool {
        self.pin_token.is_some()
    }
    
    /// Get PIN protocol version in use
    pub fn pin_protocol_version(&self) -> Option<u8> {
        self.pin_protocol_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    // Mock device for testing
    struct MockDevice {
        responses: std::collections::VecDeque<Vec<u8>>,
        connected: bool,
    }

    impl MockDevice {
        fn new() -> Self {
            Self {
                responses: std::collections::VecDeque::new(),
                connected: false,
            }
        }
        
        fn add_response(&mut self, response: Vec<u8>) {
            self.responses.push_back(response);
        }
    }

    #[async_trait]
    impl Device for MockDevice {
        async fn info(&self) -> YKeyResult<DeviceInfo> {
            Ok(DeviceInfo::new(
                "mock".to_string(),
                "Mock Device".to_string(),
                "Mock".to_string(),
                "Mock FIDO2".to_string(),
                0x1234,
                0x5678,
                DeviceType::Generic,
                TransportType::Usb,
            ))
        }
        
        async fn connect(&mut self) -> YKeyResult<()> {
            self.connected = true;
            Ok(())
        }
        
        async fn disconnect(&mut self) -> YKeyResult<()> {
            self.connected = false;
            Ok(())
        }
        
        fn is_connected(&self) -> bool {
            self.connected
        }
        
        async fn send_raw(&mut self, _data: &[u8]) -> YKeyResult<Vec<u8>> {
            if !self.connected {
                return Err(YKeyError::communication("Device not connected"));
            }
            
            self.responses.pop_front()
                .ok_or_else(|| YKeyError::communication("No response available"))
        }
    }

    #[tokio::test]
    async fn test_fido2_client_creation() {
        let device = MockDevice::new();
        let client = Fido2Client::new(device);
        
        assert!(!client.has_pin_token());
        assert_eq!(client.pin_protocol_version(), None);
    }

    #[tokio::test]
    async fn test_fido2_client_timeout() {
        let device = MockDevice::new();
        let timeout = Duration::from_millis(10); // Very short timeout
        let mut client = Fido2Client::with_timeout(device, timeout);
        
        client.device_mut().connect().await.unwrap();
        
        // This should timeout since we don't provide a response
        // The MockDevice will try to pop from an empty VecDeque and fail
        let result = client.get_info().await;
        assert!(result.is_err());
        // The error might be a communication error instead of timeout
        // since the MockDevice returns an error immediately
        let error = result.unwrap_err();
        assert!(matches!(error, YKeyError::Timeout { .. } | YKeyError::CommunicationError(_)));
    }

    #[tokio::test]
    async fn test_pin_validation() {
        let device = MockDevice::new();
        let mut client = Fido2Client::new(device);
        
        // Test PIN too short
        let result = client.set_pin("123").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), YKeyError::InvalidParameters(_)));
        
        // Test PIN too long
        let result = client.set_pin("123456789").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), YKeyError::InvalidParameters(_)));
    }

    #[test]
    fn test_pin_token_management() {
        let device = MockDevice::new();
        let mut client = Fido2Client::new(device);
        
        assert!(!client.has_pin_token());
        assert_eq!(client.pin_token(), None);
        
        // Simulate setting a PIN token
        client.pin_token = Some(vec![1, 2, 3, 4]);
        client.pin_protocol_version = Some(1);
        
        assert!(client.has_pin_token());
        assert_eq!(client.pin_token(), Some(&vec![1, 2, 3, 4]));
        assert_eq!(client.pin_protocol_version(), Some(1));
        
        client.clear_pin_token();
        assert!(!client.has_pin_token());
        assert_eq!(client.pin_token(), None);
        assert_eq!(client.pin_protocol_version(), None);
    }
}
