// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

//! Core traits for YKey device and protocol abstractions

use async_trait::async_trait;
use crate::{
    error::YKeyResult,
    types::*,
};

/// Core device abstraction trait
/// 
/// Provides the fundamental interface for hardware security key communication.
/// All device implementations must implement this trait.
#[async_trait]
pub trait Device: Send + Sync {
    /// Get device information including capabilities and metadata
    async fn info(&self) -> YKeyResult<DeviceInfo>;
    
    /// Establish connection to the hardware device
    async fn connect(&mut self) -> YKeyResult<()>;
    
    /// Disconnect from the hardware device
    async fn disconnect(&mut self) -> YKeyResult<()>;
    
    /// Check if device is currently connected
    fn is_connected(&self) -> bool;
    
    /// Send raw bytes to device and receive response
    /// 
    /// This is the lowest-level communication method.
    /// Higher-level protocols should build on top of this.
    async fn send_raw(&mut self, data: &[u8]) -> YKeyResult<Vec<u8>>;
    
    /// Get the maximum message size supported by this device
    fn max_message_size(&self) -> usize {
        7609 // Default CTAP2 max message size
    }
    
    /// Get device-specific timeout for operations
    fn operation_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

/// FIDO2/WebAuthn protocol trait
/// 
/// Implements the FIDO2 protocol operations for authentication and credential management.
#[async_trait]
pub trait Fido2Protocol: Send + Sync {
    /// Get authenticator information and capabilities
    async fn get_info(&mut self) -> YKeyResult<AuthenticatorInfo>;
    
    /// Create a new credential (registration)
    /// 
    /// This operation requires user interaction (touch/biometric).
    async fn make_credential(
        &mut self, 
        params: MakeCredentialParams
    ) -> YKeyResult<AttestationObject>;
    
    /// Get assertion for authentication
    /// 
    /// This operation may require user interaction depending on the credential.
    async fn get_assertion(
        &mut self, 
        params: GetAssertionParams
    ) -> YKeyResult<AssertionObject>;
    
    /// Reset the authenticator (factory reset)
    /// 
    /// WARNING: This will delete all credentials on the device.
    async fn reset(&mut self) -> YKeyResult<()>;
    
    /// Set PIN for the authenticator
    async fn set_pin(&mut self, pin: &str) -> YKeyResult<()>;
    
    /// Change existing PIN
    async fn change_pin(&mut self, old_pin: &str, new_pin: &str) -> YKeyResult<()>;
    
    /// Verify PIN and get PIN token
    async fn verify_pin(&mut self, pin: &str) -> YKeyResult<Vec<u8>>;
    
    /// Get next assertion (for multiple credentials)
    async fn get_next_assertion(&mut self) -> YKeyResult<AssertionObject>;
    
    /// Cancel current operation
    async fn cancel(&mut self) -> YKeyResult<()>;
}

/// Device discovery trait
/// 
/// Handles enumeration and monitoring of hardware security devices.
#[async_trait]
pub trait DeviceDiscovery: Send + Sync {
    /// Scan for available devices
    /// 
    /// Returns a list of discovered devices with their metadata.
    async fn scan(&self) -> YKeyResult<Vec<DeviceInfo>>;
    
    /// Start watching for device connection/disconnection events
    /// 
    /// Returns a stream of device events that can be monitored.
    async fn watch(&self) -> YKeyResult<DeviceEventStream>;
    
    /// Stop watching for device events
    async fn stop_watch(&self) -> YKeyResult<()>;
    
    /// Check if a specific device is currently available
    async fn is_device_available(&self, device_id: &str) -> YKeyResult<bool>;
}

/// Device creation trait (Factory pattern)
/// 
/// Used by the device factory to create specific device implementations.
pub trait DeviceCreator: Send + Sync {
    /// Create a device instance from device information
    fn create(&self, info: &DeviceInfo) -> YKeyResult<Box<dyn Device>>;
    
    /// Check if this creator supports the given device
    fn supports(&self, info: &DeviceInfo) -> bool;
    
    /// Get the priority of this creator (higher = more preferred)
    fn priority(&self) -> u32 {
        0
    }
    
    /// Get a human-readable name for this creator
    fn name(&self) -> &str;
}

/// Credential storage trait
/// 
/// Provides persistent storage for credentials and related metadata.
#[async_trait]
pub trait CredentialStore: Send + Sync {
    /// Store a credential
    async fn store(&mut self, credential: &Credential) -> YKeyResult<()>;
    
    /// Retrieve a credential by ID
    async fn get(&self, id: &CredentialId) -> YKeyResult<Option<Credential>>;
    
    /// List all stored credentials
    async fn list(&self) -> YKeyResult<Vec<Credential>>;
    
    /// List credentials for a specific relying party
    async fn list_by_rp(&self, rp_id: &str) -> YKeyResult<Vec<Credential>>;
    
    /// Delete a credential by ID
    async fn delete(&mut self, id: &CredentialId) -> YKeyResult<()>;
    
    /// Update a credential's usage information
    async fn update_usage(&mut self, id: &CredentialId) -> YKeyResult<()>;
    
    /// Clear all stored credentials
    async fn clear(&mut self) -> YKeyResult<()>;
    
    /// Get storage statistics
    async fn stats(&self) -> YKeyResult<StorageStats>;
}

/// Configuration management trait
/// 
/// Handles application and device configuration.
#[async_trait]
pub trait ConfigManager: Send + Sync {
    /// Load configuration from storage
    async fn load(&self) -> YKeyResult<AppConfig>;
    
    /// Save configuration to storage
    async fn save(&self, config: &AppConfig) -> YKeyResult<()>;
    
    /// Reset to default configuration
    async fn reset(&self) -> YKeyResult<()>;
    
    /// Validate configuration
    fn validate(&self, config: &AppConfig) -> YKeyResult<()>;
}

/// Logging and audit trail trait
/// 
/// Provides structured logging for security events and operations.
#[async_trait]
pub trait AuditLogger: Send + Sync {
    /// Log a security event
    async fn log_event(&self, event: SecurityEvent) -> YKeyResult<()>;
    
    /// Get audit log entries
    async fn get_logs(
        &self, 
        filter: LogFilter
    ) -> YKeyResult<Vec<LogEntry>>;
    
    /// Clear old log entries
    async fn cleanup(&self, older_than: chrono::DateTime<chrono::Utc>) -> YKeyResult<()>;
}

/// Transport layer trait for different communication methods
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send data over the transport
    async fn send(&mut self, data: &[u8]) -> YKeyResult<()>;
    
    /// Receive data from the transport
    async fn receive(&mut self) -> YKeyResult<Vec<u8>>;
    
    /// Check if transport is connected
    fn is_connected(&self) -> bool;
    
    /// Close the transport connection
    async fn close(&mut self) -> YKeyResult<()>;
    
    /// Get transport-specific properties
    fn properties(&self) -> TransportProperties;
}

// Additional types for the traits

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_credentials: u64,
    pub storage_used: u64,
    pub storage_available: u64,
    pub last_cleanup: chrono::DateTime<chrono::Utc>,
}

/// Application configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub auto_discovery: bool,
    pub default_timeout: u64,
    pub log_level: String,
    pub ui_theme: String,
    pub security_policies: SecurityPolicies,
}

/// Security policies configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityPolicies {
    pub require_pin: bool,
    pub require_user_verification: bool,
    pub max_pin_attempts: u32,
    pub pin_complexity: PinComplexity,
}

/// PIN complexity requirements
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PinComplexity {
    pub min_length: u32,
    pub max_length: u32,
    pub require_digits: bool,
    pub require_special_chars: bool,
}

/// Security event for audit logging
#[derive(Debug, Clone)]
pub struct SecurityEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: EventType,
    pub device_id: Option<String>,
    pub user_id: Option<String>,
    pub details: std::collections::HashMap<String, String>,
}

/// Types of security events
#[derive(Debug, Clone)]
pub enum EventType {
    DeviceConnected,
    DeviceDisconnected,
    CredentialCreated,
    AuthenticationSucceeded,
    AuthenticationFailed,
    PinChanged,
    DeviceReset,
    ConfigurationChanged,
    SecurityViolation,
}

/// Log entry structure
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub message: String,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Log filtering criteria
#[derive(Debug, Clone)]
pub struct LogFilter {
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub level: Option<LogLevel>,
    pub device_id: Option<String>,
    pub event_type: Option<EventType>,
}

/// Log levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Transport properties
#[derive(Debug, Clone)]
pub struct TransportProperties {
    pub max_packet_size: usize,
    pub supports_fragmentation: bool,
    pub connection_type: TransportType,
    pub latency_ms: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    // Mock implementations for testing

    struct MockDevice {
        connected: bool,
        info: DeviceInfo,
    }

    #[async_trait]
    impl Device for MockDevice {
        async fn info(&self) -> YKeyResult<DeviceInfo> {
            Ok(self.info.clone())
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
            Ok(vec![0x90, 0x00]) // Mock success response
        }
    }

    #[tokio::test]
    async fn test_mock_device() {
        let device_info = DeviceInfo::new(
            "mock-device".to_string(),
            "Mock Device".to_string(),
            "Mock Manufacturer".to_string(),
            "Mock Product".to_string(),
            0x1234,
            0x5678,
            DeviceType::Generic,
            TransportType::Usb,
        );

        let mut device = MockDevice {
            connected: false,
            info: device_info.clone(),
        };

        assert!(!device.is_connected());
        device.connect().await.unwrap();
        assert!(device.is_connected());

        let info = device.info().await.unwrap();
        assert_eq!(info.id, "mock-device");

        let response = device.send_raw(&[0x01, 0x02]).await.unwrap();
        assert_eq!(response, vec![0x90, 0x00]);

        device.disconnect().await.unwrap();
        assert!(!device.is_connected());
    }

    #[test]
    fn test_storage_stats() {
        let stats = StorageStats {
            total_credentials: 5,
            storage_used: 1024,
            storage_available: 8192,
            last_cleanup: chrono::Utc::now(),
        };

        assert_eq!(stats.total_credentials, 5);
        assert_eq!(stats.storage_used, 1024);
    }

    #[test]
    fn test_app_config_serialization() {
        let config = AppConfig {
            auto_discovery: true,
            default_timeout: 30,
            log_level: "info".to_string(),
            ui_theme: "dark".to_string(),
            security_policies: SecurityPolicies {
                require_pin: true,
                require_user_verification: false,
                max_pin_attempts: 3,
                pin_complexity: PinComplexity {
                    min_length: 4,
                    max_length: 8,
                    require_digits: true,
                    require_special_chars: false,
                },
            },
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.auto_discovery, true);
        assert_eq!(deserialized.security_policies.max_pin_attempts, 3);
    }
} 