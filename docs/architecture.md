# YKey Architecture Documentation

## Overview

YKey is a cross-platform hardware security key management application built with Tauri (Rust backend + TypeScript frontend). The project follows a modular architecture pattern with distinct separation of concerns across multiple Rust crates.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (TypeScript)                    │
│                     React + Tauri API                      │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                  Tauri Runtime                             │
│                 (src-tauri/)                               │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                   Rust Backend                             │
│              (Workspace Crates)                            │
└─────────────────────────────────────────────────────────────┘
```

## Crate Structure

The project is organized as a Cargo workspace with the following crates:

### 1. `ykey-core` - Core Abstractions and Types
- **Purpose**: Defines core traits, types, and error handling
- **Key Components**:
  - Device trait abstractions
  - FIDO2 protocol definitions
  - Common data structures
  - Error handling system

### 2. `ykey-device` - Device Management Layer
- **Purpose**: Handles device discovery, connection, and lifecycle management
- **Key Components**:
  - Device factory pattern for creating device instances
  - Device manager for connection lifecycle
  - Device registry and discovery interfaces

### 3. `ykey-protocol` - Protocol Implementations
- **Purpose**: Implements hardware security protocols
- **Key Components**:
  - FIDO2/WebAuthn protocol implementation
  - CTAP (Client to Authenticator Protocol) handling
  - Protocol-specific command/response handling

### 4. `ykey-platform` - Platform-Specific Implementations
- **Purpose**: Provides platform-specific device discovery and communication
- **Key Components**:
  - HID device discovery for different operating systems
  - Platform-specific transport implementations
  - Conditional compilation for different targets

### 5. `ykey-desktop` (Tauri App)
- **Purpose**: Main application entry point and Tauri integration
- **Key Components**:
  - Tauri command handlers
  - Frontend-backend communication bridge
  - Application lifecycle management

## Core Architecture Patterns

### 1. Trait-Based Design

The architecture heavily uses Rust traits for abstraction:

```rust
// Core device abstraction
#[async_trait]
pub trait Device: Send + Sync {
    async fn info(&self) -> XKeyResult<DeviceInfo>;
    async fn connect(&mut self) -> XKeyResult<()>;
    async fn disconnect(&mut self) -> XKeyResult<()>;
    fn is_connected(&self) -> bool;
    async fn send_raw(&mut self, data: &[u8]) -> XKeyResult<Vec<u8>>;
}

// Protocol abstraction
#[async_trait]
pub trait Fido2Protocol: Send + Sync {
    async fn get_info(&mut self) -> XKeyResult<AuthenticatorInfo>;
    async fn make_credential(&mut self, params: MakeCredentialParams) -> XKeyResult<AttestationObject>;
    async fn get_assertion(&mut self, params: GetAssertionParams) -> XKeyResult<AssertionObject>;
    // ... more methods
}
```

### 2. Factory Pattern

Device creation uses the factory pattern for extensibility:

```rust
pub struct DeviceFactory {
    creators: HashMap<DeviceType, Box<dyn DeviceCreator>>,
}

pub trait DeviceCreator: Send + Sync {
    fn create(&self, info: &DeviceInfo) -> XKeyResult<Box<dyn Device>>;
    fn supports(&self, info: &DeviceInfo) -> bool;
}
```

### 3. Async/Await Pattern

All I/O operations are asynchronous using Tokio:
- Device communication
- Protocol operations
- Device discovery
- File operations

### 4. Error Handling

Centralized error handling using custom error types:
- `XKeyResult<T>` for operation results
- `XKeyError` enum for different error categories
- Integration with `anyhow` and `thiserror` for error chain management

## Data Flow

### Device Discovery Flow
1. Platform-specific discovery scans for available devices
2. Device information is collected and standardized
3. Device factory creates appropriate device instances
4. Device manager maintains connection lifecycle

### Protocol Communication Flow
1. Frontend initiates operation via Tauri commands
2. Backend validates and processes request
3. Protocol layer constructs appropriate commands
4. Device layer handles raw communication
5. Response flows back through the same layers

### State Management
- Device connections managed in `Arc<RwLock<HashMap<String, Box<dyn Device>>>>`
- Async-safe shared state using Tokio primitives
- Immutable data structures where possible

## Supported Features

### Device Types
- YubiKey
- CanoKey  
- Nitrokey
- SoloKey
- Generic FIDO2 devices

### Protocols
- FIDO2/WebAuthn
- CTAP 1.0/2.0
- OATH (planned)
- PIV (planned)
- OpenPGP (planned)

### Transport Types
- USB HID
- NFC (planned)
- Bluetooth (planned)
- Hybrid transport (planned)

## Cross-Platform Support

The architecture supports multiple platforms through conditional compilation:

- **Windows**: HID API implementation
- **macOS**: IOKit-based device discovery
- **Linux**: hidraw/libusb implementation
- **Android**: NFC-based communication (planned)
- **iOS**: NFC-based communication (planned)

## Security Considerations

### Memory Safety
- Rust's ownership system prevents common memory vulnerabilities
- No unsafe code in core business logic
- Careful handling of cryptographic material

### Communication Security
- Direct hardware communication without network exposure
- Secure PIN/biometric handling
- Proper cleanup of sensitive data

### Dependency Management
- Minimal external dependencies
- Well-audited cryptographic libraries (ring, etc.)
- Regular dependency updates

## Development Guidelines

### Adding New Device Types
1. Implement `DeviceCreator` trait
2. Register with `DeviceFactory`
3. Add device-specific protocol handling
4. Update type definitions

### Adding New Protocols
1. Define protocol trait in `ykey-core`
2. Implement in `ykey-protocol`
3. Add necessary command/response types
4. Update device capabilities

### Platform Support
1. Add platform-specific discovery in `ykey-platform`
2. Implement transport layer
3. Add conditional compilation flags
4. Update build configuration

## Future Architecture Improvements

### Planned Enhancements
- Plugin system for third-party device support
- Encrypted local credential storage
- Multi-device session management
- WebAssembly protocol implementations
- Advanced logging and telemetry

### Performance Optimizations
- Connection pooling for multiple devices
- Async command batching
- Memory-mapped device communication
- Protocol command caching

---

This architecture provides a solid foundation for a secure, extensible, and maintainable hardware key management system while leveraging Rust's safety guarantees and performance characteristics. 