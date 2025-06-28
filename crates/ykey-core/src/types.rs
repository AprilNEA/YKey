// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub product_name: String,
    pub serial_number: Option<String>,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_type: DeviceType,
    pub transport: TransportType,
    pub capabilities: Vec<Capability>,
    pub firmware_version: Option<String>,
}

/// 设备类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    YubiKey,
    CanoKey,
    Nitrokey,
    SoloKey,
    Generic,
}

/// 传输类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportType {
    Usb,
    Nfc,
    Bluetooth,
    Hybrid,
}

/// 设备能力
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    Fido2,
    Fido1,
    Oath,
    Piv,
    OpenPgp,
    Otp,
}

/// 凭据信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: CredentialId,
    pub rp_id: String,
    pub user_id: Vec<u8>,
    pub user_name: String,
    pub user_display_name: String,
    pub public_key: Vec<u8>,
    pub counter: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
}

pub type CredentialId = Vec<u8>;

/// FIDO2 相关参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeCredentialParams {
    pub client_data_hash: Vec<u8>,
    pub rp: RelyingParty,
    pub user: User,
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameter>,
    pub exclude_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    pub options: MakeCredentialOptions,
    pub pin_uv_auth_param: Option<Vec<u8>>,
    pub pin_uv_auth_protocol: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAssertionParams {
    pub rp_id: String,
    pub client_data_hash: Vec<u8>,
    pub allow_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    pub options: GetAssertionOptions,
    pub pin_uv_auth_param: Option<Vec<u8>>,
    pub pin_uv_auth_protocol: Option<u8>,
}

// ... 更多类型定义
