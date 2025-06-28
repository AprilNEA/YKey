// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

//! Core types and data structures for YKey

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Device information containing metadata and capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceInfo {
    /// Unique device identifier
    pub id: String,
    /// Human-readable device name
    pub name: String,
    /// Device manufacturer
    pub manufacturer: String,
    /// Product name
    pub product_name: String,
    /// Serial number (if available)
    pub serial_number: Option<String>,
    /// USB vendor ID
    pub vendor_id: u16,
    /// USB product ID
    pub product_id: u16,
    /// Device type classification
    pub device_type: DeviceType,
    /// Communication transport method
    pub transport: TransportType,
    /// List of supported capabilities
    pub capabilities: Vec<Capability>,
    /// Firmware version (if available)
    pub firmware_version: Option<String>,
    /// When this device info was last updated
    pub last_seen: DateTime<Utc>,
}

/// Device type classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// Yubico YubiKey devices
    YubiKey,
    /// CanoKeys devices
    CanoKey,
    /// Nitrokey devices
    Nitrokey,
    /// SoloKeys devices
    SoloKey,
    /// Generic FIDO2 compatible devices
    Generic,
}

impl Default for DeviceType {
    fn default() -> Self {
        Self::Generic
    }
}

/// Communication transport methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportType {
    /// USB HID transport
    Usb,
    /// Near Field Communication
    Nfc,
    /// Bluetooth transport
    Bluetooth,
    /// Hybrid transport (multiple methods)
    Hybrid,
}

impl Default for TransportType {
    fn default() -> Self {
        Self::Usb
    }
}

/// Device capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Capability {
    /// FIDO2/WebAuthn support
    Fido2,
    /// FIDO U2F support
    Fido1,
    /// OATH TOTP/HOTP support
    Oath,
    /// PIV (Personal Identity Verification)
    Piv,
    /// OpenPGP support
    OpenPgp,
    /// OTP (One-Time Password) support
    Otp,
}

impl DeviceInfo {
    /// Create a new DeviceInfo instance
    pub fn new(
        id: String,
        name: String,
        manufacturer: String,
        product_name: String,
        vendor_id: u16,
        product_id: u16,
        device_type: DeviceType,
        transport: TransportType,
    ) -> Self {
        Self {
            id,
            name,
            manufacturer,
            product_name,
            serial_number: None,
            vendor_id,
            product_id,
            device_type,
            transport,
            capabilities: Vec::new(),
            firmware_version: None,
            last_seen: Utc::now(),
        }
    }

    /// Check if device has a specific capability
    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Add a capability to this device
    pub fn add_capability(&mut self, capability: Capability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    /// Update the last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }
}

/// Credential identifier type
pub type CredentialId = Vec<u8>;

/// Stored credential information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Credential {
    /// Credential identifier
    pub id: CredentialId,
    /// Relying party identifier
    pub rp_id: String,
    /// User identifier
    pub user_id: Vec<u8>,
    /// User name
    pub user_name: String,
    /// User display name
    pub user_display_name: String,
    /// Public key bytes
    pub public_key: Vec<u8>,
    /// Usage counter
    pub counter: u32,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last usage timestamp
    pub last_used: Option<DateTime<Utc>>,
}

/// FIDO2 MakeCredential parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeCredentialParams {
    /// Hash of client data
    pub client_data_hash: Vec<u8>,
    /// Relying party information
    pub rp: RelyingParty,
    /// User information
    pub user: User,
    /// Supported public key credential parameters
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameter>,
    /// Credentials to exclude
    pub exclude_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    /// Extensions
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    /// Options for credential creation
    pub options: MakeCredentialOptions,
    /// PIN/UV auth parameter
    pub pin_uv_auth_param: Option<Vec<u8>>,
    /// PIN/UV auth protocol version
    pub pin_uv_auth_protocol: Option<u8>,
}

/// FIDO2 GetAssertion parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAssertionParams {
    /// Relying party identifier
    pub rp_id: String,
    /// Hash of client data
    pub client_data_hash: Vec<u8>,
    /// Allowed credentials list
    pub allow_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    /// Extensions
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    /// Options for assertion
    pub options: GetAssertionOptions,
    /// PIN/UV auth parameter
    pub pin_uv_auth_param: Option<Vec<u8>>,
    /// PIN/UV auth protocol version
    pub pin_uv_auth_protocol: Option<u8>,
}

/// Relying Party information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelyingParty {
    /// Relying party identifier
    pub id: String,
    /// Relying party name
    pub name: Option<String>,
    /// Relying party icon URL
    pub icon: Option<String>,
}

/// User information for credential creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User identifier
    pub id: Vec<u8>,
    /// User name
    pub name: String,
    /// User display name
    pub display_name: String,
    /// User icon URL
    pub icon: Option<String>,
}

/// Public key credential parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyCredentialParameter {
    /// Credential type (always "public-key" for WebAuthn)
    pub cred_type: String,
    /// Algorithm identifier (COSE algorithm)
    pub alg: i64,
}

/// Public key credential descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyCredentialDescriptor {
    /// Credential type
    pub cred_type: String,
    /// Credential ID
    pub id: Vec<u8>,
    /// Allowed transports
    pub transports: Option<Vec<String>>,
}

/// Options for MakeCredential operation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MakeCredentialOptions {
    /// Require resident key
    pub rk: Option<bool>,
    /// User verification requirement
    pub uv: Option<bool>,
    /// Discourage creating new credentials
    pub up: Option<bool>,
}

/// Options for GetAssertion operation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAssertionOptions {
    /// User presence requirement
    pub up: Option<bool>,
    /// User verification requirement
    pub uv: Option<bool>,
}

/// Attestation object returned by MakeCredential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationObject {
    /// Format of attestation statement
    pub fmt: String,
    /// Attestation statement
    pub att_stmt: HashMap<String, serde_json::Value>,
    /// Authenticator data
    pub auth_data: Vec<u8>,
}

/// Assertion object returned by GetAssertion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionObject {
    /// Credential ID
    pub credential_id: Option<Vec<u8>>,
    /// Authenticator data
    pub auth_data: Vec<u8>,
    /// Assertion signature
    pub signature: Vec<u8>,
    /// User information (for resident keys)
    pub user: Option<User>,
}

/// Authenticator information from GetInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatorInfo {
    /// List of supported versions
    pub versions: Vec<String>,
    /// List of supported extensions
    pub extensions: Option<Vec<String>>,
    /// AAGUID (Authenticator Attestation GUID)
    pub aaguid: Vec<u8>,
    /// Supported options
    pub options: Option<HashMap<String, bool>>,
    /// Maximum message size
    pub max_msg_size: Option<u64>,
    /// PIN/UV protocol versions
    pub pin_uv_auth_protocols: Option<Vec<u64>>,
    /// Maximum credential count in list
    pub max_credential_count_in_list: Option<u64>,
    /// Maximum credential ID length
    pub max_credential_id_length: Option<u64>,
    /// Supported transports
    pub transports: Option<Vec<String>>,
    /// Supported algorithms
    pub algorithms: Option<Vec<PublicKeyCredentialParameter>>,
    /// Maximum serialized large blob array size
    pub max_serialized_large_blob_array: Option<u64>,
    /// Force PIN change
    pub force_pin_change: Option<bool>,
    /// Minimum PIN length
    pub min_pin_length: Option<u64>,
    /// Firmware version
    pub firmware_version: Option<u64>,
    /// Maximum credential blob length
    pub max_cred_blob_length: Option<u64>,
    /// Maximum RP IDs for SetMinPINLength
    pub max_rp_ids_for_set_min_pin_length: Option<u64>,
    /// Preferred platform UV attempts
    pub preferred_platform_uv_attempts: Option<u64>,
    /// UV modality
    pub uv_modality: Option<u64>,
    /// Certifications
    pub certifications: Option<HashMap<String, serde_json::Value>>,
    /// Remaining discoverable credentials
    pub remaining_discoverable_credentials: Option<u64>,
    /// Vendor prototype config commands
    pub vendor_prototype_config_commands: Option<Vec<u64>>,
}

/// Device event stream item
#[derive(Debug, Clone)]
pub enum DeviceEvent {
    /// Device connected
    Connected(DeviceInfo),
    /// Device disconnected
    Disconnected(String), // device_id
    /// Device error
    Error { device_id: String, error: String },
}

/// Type alias for device event stream
pub type DeviceEventStream = tokio::sync::mpsc::Receiver<DeviceEvent>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_info_creation() {
        let device = DeviceInfo::new(
            "test-id".to_string(),
            "Test Device".to_string(),
            "Test Manufacturer".to_string(),
            "Test Product".to_string(),
            0x1234,
            0x5678,
            DeviceType::Generic,
            TransportType::Usb,
        );

        assert_eq!(device.id, "test-id");
        assert_eq!(device.device_type, DeviceType::Generic);
        assert_eq!(device.transport, TransportType::Usb);
        assert!(device.capabilities.is_empty());
    }

    #[test]
    fn test_device_capabilities() {
        let mut device = DeviceInfo::new(
            "test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            0,
            0,
            DeviceType::Generic,
            TransportType::Usb,
        );

        assert!(!device.has_capability(&Capability::Fido2));
        device.add_capability(Capability::Fido2);
        assert!(device.has_capability(&Capability::Fido2));

        // Test no duplicate capabilities
        device.add_capability(Capability::Fido2);
        assert_eq!(device.capabilities.len(), 1);
    }

    #[test]
    fn test_device_type_default() {
        let default_type = DeviceType::default();
        assert_eq!(default_type, DeviceType::Generic);
    }

    #[test]
    fn test_transport_type_default() {
        let default_transport = TransportType::default();
        assert_eq!(default_transport, TransportType::Usb);
    }

    #[test]
    fn test_make_credential_options_default() {
        let options = MakeCredentialOptions::default();
        assert_eq!(options.rk, None);
        assert_eq!(options.uv, None);
        assert_eq!(options.up, None);
    }

    #[test]
    fn test_credential_creation() {
        let credential = Credential {
            id: vec![1, 2, 3, 4],
            rp_id: "example.com".to_string(),
            user_id: vec![5, 6, 7, 8],
            user_name: "test@example.com".to_string(),
            user_display_name: "Test User".to_string(),
            public_key: vec![9, 10, 11, 12],
            counter: 1,
            created_at: Utc::now(),
            last_used: None,
        };

        assert_eq!(credential.rp_id, "example.com");
        assert_eq!(credential.counter, 1);
        assert!(credential.last_used.is_none());
    }
}
