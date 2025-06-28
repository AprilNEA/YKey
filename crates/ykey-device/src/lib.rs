// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

//! Device management layer for YKey hardware security keys

use ykey_core::{traits::*, types::*, YKeyResult, YKeyError};
use async_trait::async_trait;
use std::{sync::Arc, collections::HashMap};
use tokio::sync::RwLock;

/// Device factory for creating device instances
/// 
/// Uses the factory pattern to create appropriate device implementations
/// based on device information and registered creators.
pub struct DeviceFactory {
    creators: HashMap<DeviceType, Box<dyn DeviceCreator>>,
}

// Placeholder device creators - these will be expanded into separate modules later
struct YubiKeyCreator;
struct CanoKeyCreator;
struct GenericFidoCreator;

// Implement DeviceCreator for placeholder creators
impl DeviceCreator for YubiKeyCreator {
    fn create(&self, info: &DeviceInfo) -> YKeyResult<Box<dyn Device>> {
        Ok(Box::new(MockDevice::new(info.clone())))
    }
    
    fn supports(&self, info: &DeviceInfo) -> bool {
        info.device_type == DeviceType::YubiKey
    }
    
    fn name(&self) -> &str {
        "YubiKey Creator"
    }
}

impl DeviceCreator for CanoKeyCreator {
    fn create(&self, info: &DeviceInfo) -> YKeyResult<Box<dyn Device>> {
        Ok(Box::new(MockDevice::new(info.clone())))
    }
    
    fn supports(&self, info: &DeviceInfo) -> bool {
        info.device_type == DeviceType::CanoKey
    }
    
    fn name(&self) -> &str {
        "CanoKey Creator"
    }
}

impl DeviceCreator for GenericFidoCreator {
    fn create(&self, info: &DeviceInfo) -> YKeyResult<Box<dyn Device>> {
        Ok(Box::new(MockDevice::new(info.clone())))
    }
    
    fn supports(&self, _info: &DeviceInfo) -> bool {
        true // Supports any device as fallback
    }
    
    fn name(&self) -> &str {
        "Generic FIDO Creator"
    }
}

// Mock device implementation for testing
struct MockDevice {
    info: DeviceInfo,
    connected: bool,
}

impl MockDevice {
    fn new(info: DeviceInfo) -> Self {
        Self {
            info,
            connected: false,
        }
    }
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
        if !self.connected {
            return Err(YKeyError::communication("Device not connected"));
        }
        Ok(vec![0x90, 0x00]) // Mock success response
    }
}

impl DeviceFactory {
    /// Create a new device factory with default creators
    pub fn new() -> Self {
        let mut factory = Self {
            creators: HashMap::new(),
        };
        
        // Register built-in device creators
        factory.register(DeviceType::YubiKey, Box::new(YubiKeyCreator));
        factory.register(DeviceType::CanoKey, Box::new(CanoKeyCreator));
        factory.register(DeviceType::Generic, Box::new(GenericFidoCreator));
        
        factory
    }
    
    /// Register a device creator for a specific device type
    pub fn register(&mut self, device_type: DeviceType, creator: Box<dyn DeviceCreator>) {
        self.creators.insert(device_type, creator);
    }
    
    /// Create a device instance from device information
    pub fn create_device(&self, info: &DeviceInfo) -> YKeyResult<Box<dyn Device>> {
        if let Some(creator) = self.creators.get(&info.device_type) {
            creator.create(info)
        } else {
            // Try generic creator as fallback
            if let Some(creator) = self.creators.get(&DeviceType::Generic) {
                creator.create(info)
            } else {
                Err(YKeyError::UnsupportedDevice(info.device_type.clone()))
            }
        }
    }
    
    /// Get all registered device types
    pub fn supported_device_types(&self) -> Vec<DeviceType> {
        self.creators.keys().cloned().collect()
    }
    
    /// Check if a device type is supported
    pub fn supports_device_type(&self, device_type: &DeviceType) -> bool {
        self.creators.contains_key(device_type)
    }
}

/// Device manager for handling device lifecycle and connections
/// 
/// Manages multiple devices, handles discovery, and maintains connection state.
pub struct DeviceManager {
    factory: Arc<DeviceFactory>,
    discoveries: Vec<Box<dyn DeviceDiscovery>>,
    connected_devices: Arc<RwLock<HashMap<String, Box<dyn Device>>>>,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> Self {
        Self {
            factory: Arc::new(DeviceFactory::new()),
            discoveries: Vec::new(),
            connected_devices: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a device manager with custom factory
    pub fn with_factory(factory: DeviceFactory) -> Self {
        Self {
            factory: Arc::new(factory),
            discoveries: Vec::new(),
            connected_devices: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add a device discovery mechanism
    pub fn add_discovery(&mut self, discovery: Box<dyn DeviceDiscovery>) {
        self.discoveries.push(discovery);
    }
    
    /// Scan for available devices using all registered discovery mechanisms
    pub async fn scan_devices(&self) -> YKeyResult<Vec<DeviceInfo>> {
        let mut all_devices = Vec::new();
        
        for discovery in &self.discoveries {
            let devices = discovery.scan().await?;
            all_devices.extend(devices);
        }
        
        // Remove duplicates based on device ID
        all_devices.sort_by(|a, b| a.id.cmp(&b.id));
        all_devices.dedup_by(|a, b| a.id == b.id);
        
        Ok(all_devices)
    }
    
    /// Connect to a specific device by ID
    pub async fn connect_device(&self, device_id: &str) -> YKeyResult<()> {
        let devices = self.scan_devices().await?;
        let device_info = devices.iter()
            .find(|d| d.id == device_id)
            .ok_or_else(|| YKeyError::DeviceNotFound(device_id.to_string()))?;
            
        let mut device = self.factory.create_device(device_info)?;
        device.connect().await?;
        
        let mut connected = self.connected_devices.write().await;
        connected.insert(device_id.to_string(), device);
        
        Ok(())
    }
    
    /// Disconnect a specific device by ID
    pub async fn disconnect_device(&self, device_id: &str) -> YKeyResult<()> {
        let mut connected = self.connected_devices.write().await;
        if let Some(mut device) = connected.remove(device_id) {
            device.disconnect().await?;
        }
        Ok(())
    }
    
    /// Get a reference to a connected device
    /// 
    /// Note: This returns None instead of a reference due to lifetime constraints
    /// with async RwLock. Use `with_device` for operations on connected devices.
    pub async fn is_device_connected(&self, device_id: &str) -> bool {
        let connected = self.connected_devices.read().await;
        connected.contains_key(device_id)
    }
    
    /// Execute an operation with a connected device
    pub async fn with_device<F, R>(&self, device_id: &str, f: F) -> YKeyResult<R>
    where
        F: FnOnce(&mut dyn Device) -> std::pin::Pin<Box<dyn std::future::Future<Output = YKeyResult<R>> + Send + '_>>,
    {
        let mut connected = self.connected_devices.write().await;
        if let Some(device) = connected.get_mut(device_id) {
            f(device.as_mut()).await
        } else {
            Err(YKeyError::DeviceNotFound(device_id.to_string()))
        }
    }
    
    /// Get list of connected device IDs
    pub async fn connected_device_ids(&self) -> Vec<String> {
        let connected = self.connected_devices.read().await;
        connected.keys().cloned().collect()
    }
    
    /// Get device count
    pub async fn device_count(&self) -> usize {
        let connected = self.connected_devices.read().await;
        connected.len()
    }
    
    /// Disconnect all devices
    pub async fn disconnect_all(&self) -> YKeyResult<()> {
        let mut connected = self.connected_devices.write().await;
        let device_ids: Vec<String> = connected.keys().cloned().collect();
        
        for device_id in device_ids {
            if let Some(mut device) = connected.remove(&device_id) {
                if let Err(e) = device.disconnect().await {
                    eprintln!("Failed to disconnect device {}: {}", device_id, e);
                }
            }
        }
        
        Ok(())
    }
}

impl Default for DeviceFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use ykey_core::{DeviceInfo, DeviceType, TransportType, Capability};

    // Mock discovery for testing
    struct MockDiscovery {
        devices: Vec<DeviceInfo>,
    }

    impl MockDiscovery {
        fn new(devices: Vec<DeviceInfo>) -> Self {
            Self { devices }
        }
    }

    #[async_trait]
    impl DeviceDiscovery for MockDiscovery {
        async fn scan(&self) -> YKeyResult<Vec<DeviceInfo>> {
            Ok(self.devices.clone())
        }

        async fn watch(&self) -> YKeyResult<DeviceEventStream> {
            let (_tx, rx) = tokio::sync::mpsc::channel(10);
            Ok(rx)
        }

        async fn stop_watch(&self) -> YKeyResult<()> {
            Ok(())
        }

        async fn is_device_available(&self, device_id: &str) -> YKeyResult<bool> {
            Ok(self.devices.iter().any(|d| d.id == device_id))
        }
    }

    fn create_test_device_info(id: &str, device_type: DeviceType) -> DeviceInfo {
        let mut info = DeviceInfo::new(
            id.to_string(),
            format!("Test Device {}", id),
            "Test Manufacturer".to_string(),
            "Test Product".to_string(),
            0x1234,
            0x5678,
            device_type,
            TransportType::Usb,
        );
        info.add_capability(Capability::Fido2);
        info
    }

    #[tokio::test]
    async fn test_device_factory_creation() {
        let factory = DeviceFactory::new();
        
        assert!(factory.supports_device_type(&DeviceType::YubiKey));
        assert!(factory.supports_device_type(&DeviceType::CanoKey));
        assert!(factory.supports_device_type(&DeviceType::Generic));
        
        let supported_types = factory.supported_device_types();
        assert_eq!(supported_types.len(), 3);
    }

    #[tokio::test]
    async fn test_device_creation() {
        let factory = DeviceFactory::new();
        let device_info = create_test_device_info("test-yubikey", DeviceType::YubiKey);
        
        let device = factory.create_device(&device_info).unwrap();
        let info = device.info().await.unwrap();
        assert_eq!(info.id, "test-yubikey");
        assert_eq!(info.device_type, DeviceType::YubiKey);
    }

    #[tokio::test]
    async fn test_device_manager_basic_operations() {
        let mut manager = DeviceManager::new();
        
        let devices = vec![
            create_test_device_info("device1", DeviceType::YubiKey),
            create_test_device_info("device2", DeviceType::CanoKey),
        ];
        
        let discovery = MockDiscovery::new(devices.clone());
        manager.add_discovery(Box::new(discovery));
        
        // Test device scanning
        let discovered = manager.scan_devices().await.unwrap();
        assert_eq!(discovered.len(), 2);
        assert_eq!(discovered[0].id, "device1");
        assert_eq!(discovered[1].id, "device2");
        
        // Test device count
        assert_eq!(manager.device_count().await, 0);
        
        // Test connection
        manager.connect_device("device1").await.unwrap();
        assert_eq!(manager.device_count().await, 1);
        assert!(manager.is_device_connected("device1").await);
        assert!(!manager.is_device_connected("device2").await);
        
        // Test connected device IDs
        let connected_ids = manager.connected_device_ids().await;
        assert_eq!(connected_ids.len(), 1);
        assert!(connected_ids.contains(&"device1".to_string()));
        
        // Test disconnection
        manager.disconnect_device("device1").await.unwrap();
        assert_eq!(manager.device_count().await, 0);
        assert!(!manager.is_device_connected("device1").await);
    }

    #[tokio::test]
    async fn test_device_manager_connect_multiple() {
        let mut manager = DeviceManager::new();
        
        let devices = vec![
            create_test_device_info("device1", DeviceType::YubiKey),
            create_test_device_info("device2", DeviceType::CanoKey),
            create_test_device_info("device3", DeviceType::Generic),
        ];
        
        let discovery = MockDiscovery::new(devices);
        manager.add_discovery(Box::new(discovery));
        
        // Connect multiple devices
        manager.connect_device("device1").await.unwrap();
        manager.connect_device("device2").await.unwrap();
        manager.connect_device("device3").await.unwrap();
        
        assert_eq!(manager.device_count().await, 3);
        
        // Disconnect all
        manager.disconnect_all().await.unwrap();
        assert_eq!(manager.device_count().await, 0);
    }

    #[tokio::test]
    async fn test_device_manager_error_handling() {
        let manager = DeviceManager::new();
        
        // Test connecting to non-existent device
        let result = manager.connect_device("non-existent").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), YKeyError::DeviceNotFound(_)));
        
        // Test disconnecting non-connected device (should not error)
        let result = manager.disconnect_device("non-existent").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_device_operations() {
        let device_info = create_test_device_info("mock", DeviceType::Generic);
        let mut device = MockDevice::new(device_info.clone());
        
        // Test initial state
        assert!(!device.is_connected());
        
        // Test connection
        device.connect().await.unwrap();
        assert!(device.is_connected());
        
        // Test info
        let info = device.info().await.unwrap();
        assert_eq!(info.id, "mock");
        
        // Test communication
        let response = device.send_raw(&[0x01, 0x02]).await.unwrap();
        assert_eq!(response, vec![0x90, 0x00]);
        
        // Test disconnection
        device.disconnect().await.unwrap();
        assert!(!device.is_connected());
        
        // Test communication while disconnected
        let result = device.send_raw(&[0x01, 0x02]).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_device_creators() {
        let yubikey_creator = YubiKeyCreator;
        let canokey_creator = CanoKeyCreator;
        let generic_creator = GenericFidoCreator;
        
        let yubikey_info = create_test_device_info("yubikey", DeviceType::YubiKey);
        let canokey_info = create_test_device_info("canokey", DeviceType::CanoKey);
        let generic_info = create_test_device_info("generic", DeviceType::Generic);
        
        // Test YubiKey creator
        assert!(yubikey_creator.supports(&yubikey_info));
        assert!(!yubikey_creator.supports(&canokey_info));
        assert_eq!(yubikey_creator.name(), "YubiKey Creator");
        
        // Test CanoKey creator
        assert!(canokey_creator.supports(&canokey_info));
        assert!(!canokey_creator.supports(&yubikey_info));
        assert_eq!(canokey_creator.name(), "CanoKey Creator");
        
        // Test Generic creator (supports everything)
        assert!(generic_creator.supports(&yubikey_info));
        assert!(generic_creator.supports(&canokey_info));
        assert!(generic_creator.supports(&generic_info));
        assert_eq!(generic_creator.name(), "Generic FIDO Creator");
    }

    #[tokio::test]
    async fn test_device_factory_fallback() {
        let factory = DeviceFactory::new();
        
        // Test unsupported device type falls back to generic
        let unsupported_info = create_test_device_info("unsupported", DeviceType::Nitrokey);
        let device = factory.create_device(&unsupported_info).unwrap();
        
        let info = device.info().await.unwrap();
        assert_eq!(info.device_type, DeviceType::Nitrokey);
    }
}

