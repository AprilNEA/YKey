use ykey_device::DeviceManager;
use ykey_core::{DeviceInfo, DeviceType, TransportType, Capability, YKeyResult, DeviceEventStream};
use async_trait::async_trait;
use std::process::Command;
use serde_json::Value;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

/// Device information for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendDeviceInfo {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub product_name: String,
    pub device_type: String,
    pub transport: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub capabilities: Vec<String>,
    pub is_connected: bool,
}

impl From<DeviceInfo> for FrontendDeviceInfo {
    fn from(info: DeviceInfo) -> Self {
        Self {
            id: info.id,
            name: info.name,
            manufacturer: info.manufacturer,
            product_name: info.product_name,
            device_type: format!("{:?}", info.device_type),
            transport: format!("{:?}", info.transport),
            vendor_id: info.vendor_id,
            product_id: info.product_id,
            capabilities: info.capabilities.iter().map(|c| format!("{:?}", c)).collect(),
            is_connected: false,
        }
    }
}

/// macOS-specific USB device discovery using system_profiler
pub struct MacOSUsbDiscovery;

impl MacOSUsbDiscovery {
    pub fn new() -> Self {
        Self
    }

    async fn scan_usb_devices(&self) -> YKeyResult<Vec<DeviceInfo>> {
        let output = Command::new("system_profiler")
            .args(&["SPUSBDataType", "-json"])
            .output()
            .map_err(|e| ykey_core::YKeyError::communication(&format!("Failed to run system_profiler: {}", e)))?;

        let json: Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| ykey_core::YKeyError::communication(&format!("Failed to parse system_profiler output: {}", e)))?;

        let mut devices = Vec::new();
        
        if let Some(usb_data) = json.get("SPUSBDataType") {
            Self::parse_usb_data(usb_data, &mut devices);
        }

        Ok(devices)
    }

    fn parse_usb_data(data: &Value, devices: &mut Vec<DeviceInfo>) {
        if let Some(array) = data.as_array() {
            for entry in array {
                Self::parse_usb_item(entry, devices);
            }
        } else if let Some(_obj) = data.as_object() {
            Self::parse_usb_item(data, devices);
        }
    }

    fn parse_usb_item(item: &Value, devices: &mut Vec<DeviceInfo>) {
        if let Some(product_id) = item.get("product_id") {
            if let Some(vendor_id) = item.get("vendor_id") {
                if let (Some(pid), Some(vid)) = (product_id.as_str(), vendor_id.as_str()) {
                    let pid = pid.trim_start_matches("0x");
                    let vid = vid.trim_start_matches("0x");
                    if let (Ok(product_id), Ok(vendor_id)) = (u16::from_str_radix(pid, 16), u16::from_str_radix(vid, 16)) {
                        if let Some(device_type) = Self::identify_device_type(vendor_id, product_id) {
                            let name = item.get("_name")
                                .and_then(|n| n.as_str())
                                .unwrap_or("Unknown Device")
                                .to_string();
                            let manufacturer = item.get("manufacturer")
                                .and_then(|m| m.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            let product = item.get("_name")
                                .and_then(|n| n.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            let device_id = format!("{}-{:04x}-{:04x}", format!("{:?}", device_type).to_lowercase(), vendor_id, product_id);
                            let mut info = DeviceInfo::new(
                                device_id,
                                name.clone(),
                                manufacturer,
                                product,
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
                            devices.push(info);
                        }
                    }
                }
            }
        }
        
        if let Some(children) = item.get("_items") {
            if let Some(children_array) = children.as_array() {
                for child in children_array {
                    Self::parse_usb_item(child, devices);
                }
            }
        }
    }

    fn identify_device_type(vendor_id: u16, product_id: u16) -> Option<DeviceType> {
        match (vendor_id, product_id) {
            // YubiKey devices
            (0x1050, 0x0010) | (0x1050, 0x0018) | (0x1050, 0x0030) | 
            (0x1050, 0x0407) | (0x1050, 0x0410) => Some(DeviceType::YubiKey),
            
            // CanoKey devices
            (0x20A0, 0x42D4) => Some(DeviceType::CanoKey),
            
            // Nitrokey devices
            (0x20A0, 0x42B1) | (0x20A0, 0x42B2) => Some(DeviceType::Nitrokey),
            
            // SoloKeys devices
            (0x0483, 0xA2CA) | (0x1209, 0x5070) => Some(DeviceType::SoloKey),
            
            _ => None,
        }
    }
}

#[async_trait]
impl ykey_core::traits::DeviceDiscovery for MacOSUsbDiscovery {
    async fn scan(&self) -> YKeyResult<Vec<DeviceInfo>> {
        self.scan_usb_devices().await
    }

    async fn watch(&self) -> YKeyResult<DeviceEventStream> {
        let (_tx, rx) = mpsc::channel(10);
        Ok(rx)
    }

    async fn stop_watch(&self) -> YKeyResult<()> {
        Ok(())
    }

    async fn is_device_available(&self, device_id: &str) -> YKeyResult<bool> {
        let devices = self.scan_usb_devices().await?;
        Ok(devices.iter().any(|d| d.id == device_id))
    }
}

/// Tauri Device Manager wrapper
pub struct TauriDeviceManager {
    manager: DeviceManager,
}

impl TauriDeviceManager {
    pub fn new() -> Self {
        let mut manager = DeviceManager::new();
        manager.add_discovery(Box::new(MacOSUsbDiscovery::new()));
        Self { manager }
    }

    pub async fn scan_devices(&mut self) -> Result<Vec<FrontendDeviceInfo>, String> {
        let devices = self.manager.scan_devices().await
            .map_err(|e| format!("Failed to scan devices: {}", e))?;
        
        Ok(devices.into_iter().map(FrontendDeviceInfo::from).collect())
    }

    pub async fn connect_device(&mut self, device_id: &str) -> Result<(), String> {
        self.manager.connect_device(device_id).await
            .map_err(|e| format!("Failed to connect device {}: {}", device_id, e))
    }

    pub async fn disconnect_device(&mut self, device_id: &str) -> Result<(), String> {
        self.manager.disconnect_device(device_id).await
            .map_err(|e| format!("Failed to disconnect device {}: {}", device_id, e))
    }

    pub async fn get_device_info(&mut self, device_id: &str) -> Result<FrontendDeviceInfo, String> {
        let result = self.manager.with_device(device_id, |device| {
            Box::pin(async move {
                let info = device.info().await?;
                Ok(info)
            })
        }).await;

        match result {
            Ok(info) => Ok(FrontendDeviceInfo::from(info)),
            Err(e) => Err(format!("Failed to get device info for {}: {}", device_id, e))
        }
    }

    pub async fn send_raw_command(&mut self, device_id: &str, command: Vec<u8>) -> Result<Vec<u8>, String> {
        let result = self.manager.with_device(device_id, |device| {
            Box::pin(async move {
                let response = device.send_raw(&command).await?;
                Ok(response)
            })
        }).await;

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(format!("Failed to send command to {}: {}", device_id, e))
        }
    }

    pub async fn get_connected_devices(&self) -> Vec<String> {
        self.manager.connected_device_ids().await
    }

    pub async fn disconnect_all(&mut self) -> Result<(), String> {
        self.manager.disconnect_all().await
            .map_err(|e| format!("Failed to disconnect all devices: {}", e))
    }
} 