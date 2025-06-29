# Tauri YKey Device Integration Guide

这个指南说明如何在 Tauri 应用中集成 `ykey-device` 库来管理硬件安全密钥（如 YubiKey）。

## 概览

我们已经在 Tauri 中实现了以下功能：

- 🔍 **设备扫描** - 扫描连接的 FIDO2/硬件安全密钥
- 🔌 **设备连接/断开** - 连接和断开特定设备
- ℹ️ **设备信息** - 获取设备详细信息
- 🧪 **原始命令** - 发送原始命令到设备
- 📊 **状态管理** - 跟踪连接状态

## 架构

```
Frontend (React/TypeScript)
    ↓ Tauri invoke()
Backend (Rust)
    ↓ ykey-device
Hardware Security Keys
```

## 文件结构

```
src-tauri/
├── src/
│   ├── main.rs              # Tauri 应用入口
│   ├── lib.rs               # Tauri 命令和状态管理
│   └── device_manager.rs    # 设备管理器和 macOS USB 发现
├── Cargo.toml               # 依赖配置
└── ...

src/
├── lib/
│   └── device-api.ts        # TypeScript API 包装器
├── components/
│   └── DeviceManager.tsx    # React 设备管理组件
└── ...
```

## 使用步骤

### 1. 后端集成 (Rust)

#### 添加依赖到 `src-tauri/Cargo.toml`:

```toml
[dependencies]
# YKey Crates
ykey-core = { path = "../crates/ykey-core" }
ykey-device = { path = "../crates/ykey-device" }

# Async Support
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
```

#### Tauri 命令在 `src-tauri/src/lib.rs`:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;

mod device_manager;
use device_manager::{TauriDeviceManager, FrontendDeviceInfo};

type DeviceManagerState = Arc<Mutex<TauriDeviceManager>>;

#[tauri::command]
async fn scan_devices(
    device_manager: State<'_, DeviceManagerState>,
) -> Result<Vec<FrontendDeviceInfo>, String> {
    let mut manager = device_manager.lock().await;
    manager.scan_devices().await
}

// ... 其他命令
```

### 2. 前端集成 (TypeScript)

#### TypeScript API 包装器 `src/lib/device-api.ts`:

```typescript
import { invoke } from '@tauri-apps/api/core'

export interface DeviceInfo {
  id: string
  name: string
  manufacturer: string
  // ... 其他字段
}

export class DeviceAPI {
  static async scanDevices(): Promise<DeviceInfo[]> {
    return await invoke<DeviceInfo[]>('scan_devices')
  }
  
  static async connectDevice(deviceId: string): Promise<void> {
    await invoke<void>('connect_device', { deviceId })
  }
  
  // ... 其他方法
}
```

#### React 组件 `src/components/DeviceManager.tsx`:

```tsx
import DeviceAPI, { DeviceInfo } from '../lib/device-api'

export const DeviceManager: React.FC = () => {
  const [devices, setDevices] = useState<DeviceInfo[]>([])
  
  const handleScanDevices = async () => {
    const discoveredDevices = await DeviceAPI.scanDevices()
    setDevices(discoveredDevices)
  }
  
  // ... 其他处理函数
}
```

## 可用的 Tauri 命令

| 命令 | 参数 | 返回值 | 描述 |
|------|------|-------|------|
| `scan_devices` | - | `DeviceInfo[]` | 扫描可用设备 |
| `connect_device` | `deviceId: string` | `void` | 连接到设备 |
| `disconnect_device` | `deviceId: string` | `void` | 断开设备连接 |
| `get_device_info` | `deviceId: string` | `DeviceInfo` | 获取设备信息 |
| `send_raw_command` | `deviceId: string, command: number[]` | `number[]` | 发送原始命令 |
| `get_connected_devices` | - | `string[]` | 获取已连接设备ID |
| `disconnect_all_devices` | - | `void` | 断开所有设备 |

## DeviceInfo 接口

```typescript
interface DeviceInfo {
  id: string              // 唯一设备标识符
  name: string           // 设备名称
  manufacturer: string   // 制造商
  product_name: string   // 产品名称
  device_type: string    // 设备类型 (YubiKey, CanoKey 等)
  transport: string      // 传输类型 (Usb)
  vendor_id: number      // 厂商ID
  product_id: number     // 产品ID
  capabilities: string[] // 功能列表 (Fido2, Oath 等)
  is_connected: boolean  // 连接状态
}
```

## 使用示例

### 基本设备扫描和连接:

```typescript
// 扫描设备
const devices = await DeviceAPI.scanDevices()
console.log('Found devices:', devices)

// 连接第一个设备
if (devices.length > 0) {
  await DeviceAPI.connectDevice(devices[0].id)
  console.log('Connected to:', devices[0].name)
}

// 获取设备信息
const deviceInfo = await DeviceAPI.getDeviceInfo(devices[0].id)
console.log('Device info:', deviceInfo)

// 断开连接
await DeviceAPI.disconnectDevice(devices[0].id)
```

### 发送原始命令:

```typescript
// 发送测试命令 (4个零字节)
const deviceId = 'your-device-id'
const command = [0x00, 0x00, 0x00, 0x00]
const response = await DeviceAPI.sendRawCommand(deviceId, command)
console.log('Response:', response)
```

## 平台支持

目前的实现针对 **macOS** 进行了优化，使用 `system_profiler` 进行 USB 设备发现。

### 扩展到其他平台:

1. **Windows**: 可以使用 WMI 或 Windows API
2. **Linux**: 可以使用 `lsusb` 或直接读取 `/sys/bus/usb/devices/`

## 错误处理

所有 API 调用都会抛出带有描述性错误消息的异常：

```typescript
try {
  const devices = await DeviceAPI.scanDevices()
} catch (error) {
  console.error('扫描设备失败:', error.message)
}
```

## 开发和调试

### 启动开发环境:

```bash
# 启动 Tauri 开发服务器
npm run tauri dev

# 或者
cargo tauri dev
```

### 查看日志:

- 前端日志: 浏览器开发者工具
- 后端日志: Tauri 控制台输出

## 注意事项

1. **权限**: 访问 USB 设备可能需要管理员权限
2. **设备占用**: 确保设备没有被其他应用程序占用
3. **连接状态**: 始终检查设备连接状态再执行操作
4. **错误处理**: 实现适当的错误处理和用户反馈

## 下一步

- 实现设备事件监听 (插入/拔出)
- 添加 FIDO2/WebAuthn 操作
- 实现 OATH-TOTP 管理
- 添加 PIV 证书管理
- 支持更多设备类型 