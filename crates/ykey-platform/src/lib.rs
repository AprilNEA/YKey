// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

//! Platform-specific implementations for YKey hardware security keys
//! 
//! This crate provides platform-specific device discovery and communication
//! implementations for different operating systems.

use ykey_core::{traits::*, types::*, YKeyResult, YKeyError};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::mpsc;

// Platform-specific modules will be implemented in future versions
// For now, we use mock implementations

/// Create platform-specific device discovery
/// 
/// Returns the most appropriate device discovery implementation for the current platform.
/// For now, this returns a mock implementation while platform-specific modules are being developed.
pub fn create_platform_discovery() -> Box<dyn DeviceDiscovery> {
    // TODO: Implement platform-specific discovery
    // For now, return mock discovery for all platforms
    Box::new(MockDiscovery::new())
}

/// Mock discovery implementation for unsupported platforms or testing
pub struct MockDiscovery {
    devices: Vec<DeviceInfo>,
}

impl MockDiscovery {
    /// Create a new mock discovery
    pub fn new() -> Self {
        Self {
            devices: vec![
                create_mock_device("mock-yubikey-1", DeviceType::YubiKey, 0x1050, 0x0407),
                create_mock_device("mock-canokey-1", DeviceType::CanoKey, 0x20A0, 0x42D4),
            ],
        }
    }
    
    /// Create mock discovery with custom devices
    pub fn with_devices(devices: Vec<DeviceInfo>) -> Self {
        Self { devices }
    }
}

#[async_trait]
impl DeviceDiscovery for MockDiscovery {
    async fn scan(&self) -> YKeyResult<Vec<DeviceInfo>> {
        Ok(self.devices.clone())
    }
    
    async fn watch(&self) -> YKeyResult<DeviceEventStream> {
        let (_tx, rx) = mpsc::channel(10);
        Ok(rx)
    }
    
    async fn stop_watch(&self) -> YKeyResult<()> {
        Ok(())
    }
    
    async fn is_device_available(&self, device_id: &str) -> YKeyResult<bool> {
        Ok(self.devices.iter().any(|d| d.id == device_id))
    }
}

impl Default for MockDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create mock device info
fn create_mock_device(id: &str, device_type: DeviceType, vendor_id: u16, product_id: u16) -> DeviceInfo {
    let (manufacturer, product_name) = match device_type {
        DeviceType::YubiKey => ("Yubico", "YubiKey"),
        DeviceType::CanoKey => ("CanoKeys", "CanoKey"),
        DeviceType::Nitrokey => ("Nitrokey", "Nitrokey"),
        DeviceType::SoloKey => ("SoloKeys", "Solo"),
        DeviceType::Generic => ("Generic", "FIDO2 Key"),
    };
    
    let mut info = DeviceInfo::new(
        id.to_string(),
        format!("{} {}", manufacturer, product_name),
        manufacturer.to_string(),
        product_name.to_string(),
        vendor_id,
        product_id,
        device_type,
        TransportType::Usb,
    );
    
    info.add_capability(Capability::Fido2);
    if matches!(device_type, DeviceType::YubiKey) {
        info.add_capability(Capability::Fido1);
        info.add_capability(Capability::Oath);
        info.add_capability(Capability::Piv);
        info.add_capability(Capability::Otp);
    }
    
    info
}

/// Common FIDO2 device vendor/product ID combinations
pub struct FidoDeviceIds;

impl FidoDeviceIds {
    /// Known FIDO2 device vendor/product ID combinations
    pub const KNOWN_DEVICES: &'static [(u16, u16, DeviceType)] = &[
        // Yubico devices
        (0x1050, 0x0010, DeviceType::YubiKey), // YubiKey
        (0x1050, 0x0018, DeviceType::YubiKey), // YubiKey (various models)
        (0x1050, 0x0030, DeviceType::YubiKey), // YubiKey 4/5 series
        (0x1050, 0x0407, DeviceType::YubiKey), // YubiKey 5 series
        (0x1050, 0x0410, DeviceType::YubiKey), // YubiKey Plus
        
        // CanoKeys devices
        (0x20A0, 0x42D4, DeviceType::CanoKey), // CanoKey
        
        // Nitrokey devices
        (0x20A0, 0x42B1, DeviceType::Nitrokey), // Nitrokey
        (0x20A0, 0x42B2, DeviceType::Nitrokey), // Nitrokey FIDO2
        
        // SoloKeys devices
        (0x0483, 0xA2CA, DeviceType::SoloKey), // Solo 1
        (0x1209, 0x5070, DeviceType::SoloKey), // Solo 2
    ];
    
    /// Check if a vendor/product ID combination is a known FIDO2 device
    pub fn is_known_fido_device(vendor_id: u16, product_id: u16) -> Option<DeviceType> {
        Self::KNOWN_DEVICES
            .iter()
            .find(|(vid, pid, _)| *vid == vendor_id && *pid == product_id)
            .map(|(_, _, device_type)| *device_type)
    }
    
    /// Get all known vendor IDs
    pub fn known_vendor_ids() -> Vec<u16> {
        let mut vendor_ids: Vec<u16> = Self::KNOWN_DEVICES.iter().map(|(vid, _, _)| *vid).collect();
        vendor_ids.sort_unstable();
        vendor_ids.dedup();
        vendor_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_mock_discovery() {
        let discovery = MockDiscovery::new();
        
        let devices = discovery.scan().await.unwrap();
        assert_eq!(devices.len(), 2);
        
        assert_eq!(devices[0].device_type, DeviceType::YubiKey);
        assert_eq!(devices[1].device_type, DeviceType::CanoKey);
        
        // Test device availability
        assert!(discovery.is_device_available("mock-yubikey-1").await.unwrap());
        assert!(!discovery.is_device_available("non-existent").await.unwrap());
    }
    
    #[tokio::test]
    async fn test_custom_mock_discovery() {
        let custom_devices = vec![
            create_mock_device("custom-1", DeviceType::Nitrokey, 0x20A0, 0x42B1),
        ];
        
        let discovery = MockDiscovery::with_devices(custom_devices);
        let devices = discovery.scan().await.unwrap();
        
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].device_type, DeviceType::Nitrokey);
        assert_eq!(devices[0].id, "custom-1");
    }
    
    #[test]
    fn test_fido_device_ids() {
        // Test known devices
        assert_eq!(
            FidoDeviceIds::is_known_fido_device(0x1050, 0x0407),
            Some(DeviceType::YubiKey)
        );
        assert_eq!(
            FidoDeviceIds::is_known_fido_device(0x20A0, 0x42D4),
            Some(DeviceType::CanoKey)
        );
        
        // Test unknown device
        assert_eq!(FidoDeviceIds::is_known_fido_device(0xFFFF, 0xFFFF), None);
        
        // Test vendor IDs
        let vendor_ids = FidoDeviceIds::known_vendor_ids();
        assert!(vendor_ids.contains(&0x1050)); // Yubico
        assert!(vendor_ids.contains(&0x20A0)); // Various FIDO2 devices
    }
    
    #[test]
    fn test_create_mock_device() {
        let device = create_mock_device("test-id", DeviceType::YubiKey, 0x1050, 0x0407);
        
        assert_eq!(device.id, "test-id");
        assert_eq!(device.device_type, DeviceType::YubiKey);
        assert_eq!(device.vendor_id, 0x1050);
        assert_eq!(device.product_id, 0x0407);
        assert!(device.has_capability(&Capability::Fido2));
        assert!(device.has_capability(&Capability::Fido1));
        assert!(device.has_capability(&Capability::Oath));
    }
    
    #[test]
    fn test_platform_discovery_creation() {
        let _discovery = create_platform_discovery();
        // Should create without panicking - if we reach this point, it worked
        assert!(true);
    }
}
