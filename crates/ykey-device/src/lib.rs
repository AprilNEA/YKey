// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

// crates/xkey-device/src/lib.rs
pub mod factory;
pub mod registry;
pub mod manager;

use xkey_core::{traits::*, types::*, error::XKeyResult};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 设备工厂
pub struct DeviceFactory {
    creators: HashMap<DeviceType, Box<dyn DeviceCreator>>,
}

/// 设备创建器特征
pub trait DeviceCreator: Send + Sync {
    fn create(&self, info: &DeviceInfo) -> XKeyResult<Box<dyn Device>>;
    fn supports(&self, info: &DeviceInfo) -> bool;
}

impl DeviceFactory {
    pub fn new() -> Self {
        let mut factory = Self {
            creators: HashMap::new(),
        };
        
        // 注册内置设备创建器
        factory.register(DeviceType::YubiKey, Box::new(YubiKeyCreator));
        factory.register(DeviceType::CanoKey, Box::new(CanoKeyCreator));
        factory.register(DeviceType::Generic, Box::new(GenericFidoCreator));
        
        factory
    }
    
    pub fn register(&mut self, device_type: DeviceType, creator: Box<dyn DeviceCreator>) {
        self.creators.insert(device_type, creator);
    }
    
    pub fn create_device(&self, info: &DeviceInfo) -> XKeyResult<Box<dyn Device>> {
        if let Some(creator) = self.creators.get(&info.device_type) {
            creator.create(info)
        } else {
            // 尝试通用创建器
            if let Some(creator) = self.creators.get(&DeviceType::Generic) {
                creator.create(info)
            } else {
                Err(XKeyError::UnsupportedDevice(info.device_type.clone()))
            }
        }
    }
}

/// 设备管理器
pub struct DeviceManager {
    factory: Arc<DeviceFactory>,
    discoveries: Vec<Box<dyn DeviceDiscovery>>,
    connected_devices: Arc<RwLock<HashMap<String, Box<dyn Device>>>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            factory: Arc::new(DeviceFactory::new()),
            discoveries: Vec::new(),
            connected_devices: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn add_discovery(&mut self, discovery: Box<dyn DeviceDiscovery>) {
        self.discoveries.push(discovery);
    }
    
    pub async fn scan_devices(&self) -> XKeyResult<Vec<DeviceInfo>> {
        let mut all_devices = Vec::new();
        
        for discovery in &self.discoveries {
            let devices = discovery.scan().await?;
            all_devices.extend(devices);
        }
        
        // 去重
        all_devices.sort_by(|a, b| a.id.cmp(&b.id));
        all_devices.dedup_by(|a, b| a.id == b.id);
        
        Ok(all_devices)
    }
    
    pub async fn connect_device(&self, device_id: &str) -> XKeyResult<()> {
        let devices = self.scan_devices().await?;
        let device_info = devices.iter()
            .find(|d| d.id == device_id)
            .ok_or(XKeyError::DeviceNotFound(device_id.to_string()))?;
            
        let mut device = self.factory.create_device(device_info)?;
        device.connect().await?;
        
        let mut connected = self.connected_devices.write().await;
        connected.insert(device_id.to_string(), device);
        
        Ok(())
    }
    
    pub async fn get_device(&self, device_id: &str) -> XKeyResult<Option<&dyn Device>> {
        let connected = self.connected_devices.read().await;
        Ok(connected.get(device_id).map(|d| d.as_ref()))
    }
}
