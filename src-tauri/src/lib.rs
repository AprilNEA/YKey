// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;

mod device_manager;
use device_manager::{TauriDeviceManager, FrontendDeviceInfo};

// Global device manager state
type DeviceManagerState = Arc<Mutex<TauriDeviceManager>>;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Scan for available devices
#[tauri::command]
async fn scan_devices(
    device_manager: State<'_, DeviceManagerState>,
) -> Result<Vec<FrontendDeviceInfo>, String> {
    let mut manager = device_manager.lock().await;
    manager.scan_devices().await
}

/// Connect to a specific device
#[tauri::command]
async fn connect_device(
    device_id: String,
    device_manager: State<'_, DeviceManagerState>,
) -> Result<(), String> {
    let mut manager = device_manager.lock().await;
    manager.connect_device(&device_id).await
}

/// Disconnect from a specific device
#[tauri::command]
async fn disconnect_device(
    device_id: String,
    device_manager: State<'_, DeviceManagerState>,
) -> Result<(), String> {
    let mut manager = device_manager.lock().await;
    manager.disconnect_device(&device_id).await
}

/// Get detailed information about a connected device
#[tauri::command]
async fn get_device_info(
    device_id: String,
    device_manager: State<'_, DeviceManagerState>,
) -> Result<FrontendDeviceInfo, String> {
    let mut manager = device_manager.lock().await;
    manager.get_device_info(&device_id).await
}

/// Send raw command to device
#[tauri::command]
async fn send_raw_command(
    device_id: String,
    command: Vec<u8>,
    device_manager: State<'_, DeviceManagerState>,
) -> Result<Vec<u8>, String> {
    let mut manager = device_manager.lock().await;
    manager.send_raw_command(&device_id, command).await
}

/// Get list of currently connected device IDs
#[tauri::command]
async fn get_connected_devices(
    device_manager: State<'_, DeviceManagerState>,
) -> Result<Vec<String>, String> {
    let manager = device_manager.lock().await;
    Ok(manager.get_connected_devices().await)
}

/// Disconnect all devices
#[tauri::command]
async fn disconnect_all_devices(
    device_manager: State<'_, DeviceManagerState>,
) -> Result<(), String> {
    let mut manager = device_manager.lock().await;
    manager.disconnect_all().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(TauriDeviceManager::new())))
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_devices,
            connect_device,
            disconnect_device,
            get_device_info,
            send_raw_command,
            get_connected_devices,
            disconnect_all_devices
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
