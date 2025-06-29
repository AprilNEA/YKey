use ykey_device::DeviceManager;
use ykey_core::{DeviceInfo, DeviceType, TransportType, Capability, YKeyResult, DeviceEvent, DeviceEventStream};
use async_trait::async_trait;
use std::process::Command;
use serde_json::Value;
use tokio::sync::mpsc;

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

        // Added: Print raw JSON structure for debugging
        println!("[DEBUG] system_profiler SPUSBDataType -json output: \n{}", serde_json::to_string_pretty(&json).unwrap_or_default());

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
            // Compatible with direct object case
            Self::parse_usb_item(data, devices);
        }
    }

    fn parse_usb_item(item: &Value, devices: &mut Vec<DeviceInfo>) {
        // First try to parse current item as device
        if let Some(product_id) = item.get("product_id") {
            if let Some(vendor_id) = item.get("vendor_id") {
                if let (Some(pid), Some(vid)) = (product_id.as_str(), vendor_id.as_str()) {
                    // Fix: Remove 0x prefix
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
                            println!("Found device: {} (VID: 0x{:04x}, PID: 0x{:04x})", name, vendor_id, product_id);
                        }
                    }
                }
            }
        }
        // Recursively traverse all child _items
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

#[tokio::main]
async fn main() -> YKeyResult<()> {
    println!("🔍 Scanning USB devices for YubiKey...");
    println!("==================================");

    // Use MacOSUsbDiscovery as DeviceManager discovery mechanism
    let mut manager = DeviceManager::new();
    manager.add_discovery(Box::new(MacOSUsbDiscovery::new()));

    // Scan USB devices
    let devices = manager.scan_devices().await?;
    
    if devices.is_empty() {
        println!("❌ No FIDO2 devices found");
        println!("\nPlease ensure:");
        println!("1. YubiKey is properly inserted into USB port");
        println!("2. Device is not occupied by other programs");
        println!("3. System has recognized the device");
        return Ok(());
    }

    println!("\n✅ Found {} FIDO2 device(s):", devices.len());
    println!("==================================");

    // Display device information
    for (i, device) in devices.iter().enumerate() {
        println!("\nDevice {}:", i + 1);
        println!("  ID: {}", device.id);
        println!("  Name: {}", device.name);
        println!("  Manufacturer: {}", device.manufacturer);
        println!("  Product: {}", device.product_name);
        println!("  Type: {:?}", device.device_type);
        println!("  Transport: {:?}", device.transport);
        println!("  VID: 0x{:04x}", device.vendor_id);
        println!("  PID: 0x{:04x}", device.product_id);
        println!("  Capabilities: {:?}", device.capabilities);
    }

    // Create device manager and try to connect
    println!("\n🔌 Attempting to connect devices...");
    println!("==================================");

    for device in &devices {
        println!("\nAttempting to connect device: {}", device.name);
        
        match manager.connect_device(&device.id).await {
            Ok(()) => {
                println!("✅ Successfully connected to {}", device.name);
                
                // Test device communication
                let result = manager.with_device(&device.id, |device| {
                    Box::pin(async move {
                        let info = device.info().await?;
                        println!("  Device info: {:?}", info);
                        
                        // Try sending test command
                        let response = device.send_raw(&[0x00, 0x00, 0x00, 0x00]).await?;
                        println!("  Test response: {:?}", response);
                        
                        Ok::<(), ykey_core::YKeyError>(())
                    })
                }).await;
                
                match result {
                    Ok(()) => println!("✅ Device communication test successful"),
                    Err(e) => println!("❌ Device communication test failed: {}", e),
                }
            }
            Err(e) => {
                println!("❌ Connection failed: {}", e);
            }
        }
    }

    // Display connection status
    let connected_count = manager.device_count().await;
    let connected_ids = manager.connected_device_ids().await;
    
    println!("\n📊 Connection Status:");
    println!("==================================");
    println!("Connected device count: {}", connected_count);
    if !connected_ids.is_empty() {
        println!("Connected device IDs: {:?}", connected_ids);
    }

    // Clean up connections
    println!("\n🧹 Cleaning up connections...");
    manager.disconnect_all().await?;
    println!("✅ All devices disconnected");

    Ok(())
} 