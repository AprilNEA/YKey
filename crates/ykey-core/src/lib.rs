// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

// crates/xkey-core/src/lib.rs
pub mod device;
pub mod error;
pub mod types;
pub mod traits;
pub mod config;

// crates/xkey-core/src/traits.rs
use async_trait::async_trait;
use crate::{error::XKeyResult, types::*};

/// 设备抽象特征
#[async_trait]
pub trait Device: Send + Sync {
    /// 设备信息
    async fn info(&self) -> XKeyResult<DeviceInfo>;
    
    /// 连接设备
    async fn connect(&mut self) -> XKeyResult<()>;
    
    /// 断开连接
    async fn disconnect(&mut self) -> XKeyResult<()>;
    
    /// 检查是否已连接
    fn is_connected(&self) -> bool;
    
    /// 发送原始数据
    async fn send_raw(&mut self, data: &[u8]) -> XKeyResult<Vec<u8>>;
}

/// FIDO2 协议特征
#[async_trait]
pub trait Fido2Protocol: Send + Sync {
    /// 获取认证器信息
    async fn get_info(&mut self) -> XKeyResult<AuthenticatorInfo>;
    
    /// 创建凭据
    async fn make_credential(
        &mut self, 
        params: MakeCredentialParams
    ) -> XKeyResult<AttestationObject>;
    
    /// 获取断言
    async fn get_assertion(
        &mut self, 
        params: GetAssertionParams
    ) -> XKeyResult<AssertionObject>;
    
    /// 重置设备
    async fn reset(&mut self) -> XKeyResult<()>;
    
    /// PIN 相关操作
    async fn set_pin(&mut self, pin: &str) -> XKeyResult<()>;
    async fn change_pin(&mut self, old_pin: &str, new_pin: &str) -> XKeyResult<()>;
    async fn verify_pin(&mut self, pin: &str) -> XKeyResult<bool>;
}

/// 设备发现特征
#[async_trait]
pub trait DeviceDiscovery: Send + Sync {
    /// 扫描设备
    async fn scan(&self) -> XKeyResult<Vec<DeviceInfo>>;
    
    /// 监听设备插拔事件
    async fn watch(&self) -> XKeyResult<DeviceEventStream>;
}

/// 凭据存储特征
#[async_trait]
pub trait CredentialStore: Send + Sync {
    /// 存储凭据
    async fn store(&mut self, credential: &Credential) -> XKeyResult<()>;
    
    /// 获取凭据
    async fn get(&self, id: &CredentialId) -> XKeyResult<Option<Credential>>;
    
    /// 列出所有凭据
    async fn list(&self) -> XKeyResult<Vec<Credential>>;
    
    /// 删除凭据
    async fn delete(&mut self, id: &CredentialId) -> XKeyResult<()>;
}
