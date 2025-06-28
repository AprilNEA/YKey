# Development Guide

## Getting Started

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/) (minimum version: 1.70.0)
- **Node.js**: Version 18+ for the frontend (installed via [pnpm](https://pnpm.io/))
- **System Dependencies**:
  - **Linux**: `libudev-dev`, `libusb-1.0-0-dev`, `pkg-config`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Project Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd ykey
```

2. Install frontend dependencies:
```bash
pnpm install
```

3. Build the Rust workspace:
```bash
cargo build
```

4. Run the development server:
```bash
pnpm tauri dev
```

## Project Structure

```
ykey/
├── crates/                     # Rust workspace crates
│   ├── ykey-core/             # Core abstractions and traits
│   ├── ykey-device/           # Device management layer
│   ├── ykey-protocol/         # Protocol implementations
│   └── ykey-platform/         # Platform-specific code
├── src-tauri/                 # Tauri application
├── src/                       # React frontend
├── docs/                      # Documentation
└── Cargo.toml                 # Workspace configuration
```

## Coding Standards

### Rust Code

#### Style Guidelines
- Follow official Rust formatting with `rustfmt`
- Use `clippy` for linting
- Maximum line length: 100 characters
- Use descriptive variable and function names

#### Error Handling
- Always use `XKeyResult<T>` for fallible operations
- Create specific error variants for different failure modes
- Include context in error messages

#### Async Programming
- All I/O operations must be async
- Use `tokio` runtime features appropriately
- Avoid blocking operations in async contexts

#### Documentation
- Document all public APIs with `///` comments
- Include examples for complex functions
- Document safety invariants for any unsafe code

#### Example:
```rust
/// Connects to a hardware security device
/// 
/// # Arguments
/// * `device_id` - Unique identifier for the device
/// 
/// # Returns
/// * `Ok(())` if connection successful
/// * `Err(XKeyError::DeviceNotFound)` if device doesn't exist
/// * `Err(XKeyError::CommunicationError)` if connection fails
/// 
/// # Example
/// ```rust
/// let manager = DeviceManager::new();
/// manager.connect_device("yubikey_12345").await?;
/// ```
pub async fn connect_device(&self, device_id: &str) -> XKeyResult<()> {
    // Implementation
}
```

### TypeScript Code

#### Style Guidelines
- Use TypeScript strict mode
- Follow ESLint configuration
- Use functional components with hooks
- Prefer `const` assertions for immutable data

#### Component Structure
- One component per file
- Export as default
- Use descriptive prop types
- Include JSDoc comments for complex props

## Architecture Principles

### Separation of Concerns
- **Core**: Abstract traits and types only
- **Device**: Device management and lifecycle
- **Protocol**: Protocol-specific implementations
- **Platform**: OS-specific code with conditional compilation

### Dependency Management
- Core crates should have minimal dependencies
- Platform-specific dependencies only in platform crates
- Use workspace dependencies for shared packages

### Error Handling Strategy
- Fail fast with descriptive errors
- Use structured error types with context
- Log errors at appropriate levels
- Graceful degradation where possible

### Testing Strategy
- Unit tests for individual functions
- Integration tests for component interactions
- Mock hardware devices for testing
- Property-based testing for protocol validation

## Adding New Features

### Adding a New Device Type

1. **Define the device type** in `ykey-core/src/types.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    YubiKey,
    CanoKey,
    Nitrokey,
    SoloKey,
    NewDevice,  // Add your device type
    Generic,
}
```

2. **Implement DeviceCreator** in `ykey-device/src/creators/`:
```rust
pub struct NewDeviceCreator;

impl DeviceCreator for NewDeviceCreator {
    fn create(&self, info: &DeviceInfo) -> XKeyResult<Box<dyn Device>> {
        Ok(Box::new(NewDevice::new(info)?))
    }
    
    fn supports(&self, info: &DeviceInfo) -> bool {
        info.device_type == DeviceType::NewDevice
    }
}
```

3. **Implement Device trait**:
```rust
pub struct NewDevice {
    info: DeviceInfo,
    connected: bool,
}

#[async_trait]
impl Device for NewDevice {
    async fn info(&self) -> XKeyResult<DeviceInfo> {
        Ok(self.info.clone())
    }
    
    async fn connect(&mut self) -> XKeyResult<()> {
        // Device-specific connection logic
        self.connected = true;
        Ok(())
    }
    
    // ... implement other methods
}
```

4. **Register in DeviceFactory**:
```rust
factory.register(DeviceType::NewDevice, Box::new(NewDeviceCreator));
```

### Adding a New Protocol

1. **Define protocol trait** in `ykey-core/src/traits.rs`:
```rust
#[async_trait]
pub trait NewProtocol: Send + Sync {
    async fn protocol_operation(&mut self, params: NewParams) -> XKeyResult<NewResponse>;
}
```

2. **Implement protocol** in `ykey-protocol/src/`:
```rust
pub struct NewProtocolClient<D: Device> {
    device: D,
}

#[async_trait]
impl<D: Device> NewProtocol for NewProtocolClient<D> {
    async fn protocol_operation(&mut self, params: NewParams) -> XKeyResult<NewResponse> {
        // Protocol implementation
    }
}
```

3. **Add to device capabilities**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    Fido2,
    Oath,
    Piv,
    OpenPgp,
    Otp,
    NewProtocol,  // Add new capability
}
```

### Adding Platform Support

1. **Create platform module** in `ykey-platform/src/`:
```rust
// ykey-platform/src/new_platform.rs
pub struct NewPlatformDiscovery;

#[async_trait]
impl DeviceDiscovery for NewPlatformDiscovery {
    async fn scan(&self) -> XKeyResult<Vec<DeviceInfo>> {
        // Platform-specific device scanning
    }
    
    async fn watch(&self) -> XKeyResult<DeviceEventStream> {
        // Platform-specific device monitoring
    }
}
```

2. **Add conditional compilation** in `ykey-platform/src/lib.rs`:
```rust
#[cfg(target_os = "new_platform")]
pub mod new_platform;

pub fn create_platform_discovery() -> Box<dyn DeviceDiscovery> {
    #[cfg(target_os = "new_platform")]
    return Box::new(new_platform::NewPlatformDiscovery::new());
    
    // ... existing platforms
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p ykey-core

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

### Writing Tests

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_device_connection() {
        let mut device = MockDevice::new();
        assert!(device.connect().await.is_ok());
        assert!(device.is_connected());
    }
}
```

#### Integration Tests
```rust
// tests/integration_test.rs
use ykey_device::DeviceManager;
use ykey_platform::create_platform_discovery;

#[tokio::test]
async fn test_device_discovery() {
    let mut manager = DeviceManager::new();
    manager.add_discovery(create_mock_discovery());
    
    let devices = manager.scan_devices().await.unwrap();
    assert!(!devices.is_empty());
}
```

### Mocking Hardware Devices

Create mock implementations for testing:

```rust
pub struct MockDevice {
    info: DeviceInfo,
    connected: bool,
    responses: Vec<Vec<u8>>,
}

#[async_trait]
impl Device for MockDevice {
    async fn send_raw(&mut self, _data: &[u8]) -> XKeyResult<Vec<u8>> {
        Ok(self.responses.pop().unwrap_or_default())
    }
    
    // ... other implementations
}
```

## Debugging

### Logging
Use structured logging with the `tracing` crate:

```rust
use tracing::{info, warn, error, debug, trace};

#[tracing::instrument]
async fn connect_device(&self, device_id: &str) -> XKeyResult<()> {
    info!("Attempting to connect to device: {}", device_id);
    
    match self.scan_devices().await {
        Ok(devices) => {
            debug!("Found {} devices", devices.len());
            // ... connection logic
        }
        Err(e) => {
            error!("Failed to scan devices: {}", e);
            return Err(e);
        }
    }
}
```

### Enable Debug Logging
```bash
RUST_LOG=debug cargo run
RUST_LOG=ykey_device=trace cargo run
```

### Hardware Debugging
- Use `hidapi` debug features for HID communication
- Enable CTAP debug logging
- Monitor USB traffic with platform tools

## Performance Considerations

### Async Best Practices
- Use `tokio::spawn` for CPU-intensive tasks
- Avoid blocking operations in async contexts
- Use `Arc<RwLock<T>>` for shared mutable state
- Prefer message passing over shared state

### Memory Management
- Use `Box<dyn Trait>` for trait objects
- Prefer `&str` over `String` for temporary data
- Use `Cow<str>` for potentially owned strings
- Minimize large stack allocations

### Device Communication
- Batch multiple operations when possible
- Implement connection pooling for multiple devices
- Cache device capabilities and info
- Use timeouts for hardware operations

## Security Guidelines

### Cryptographic Operations
- Use well-audited cryptographic libraries (`ring`, `rustcrypto`)
- Never implement custom cryptographic primitives
- Clear sensitive data from memory after use
- Use constant-time operations for secret comparisons

### Input Validation
- Validate all input from frontend
- Sanitize data before sending to hardware
- Check buffer bounds for device communication
- Validate protocol message formats

### Error Information
- Don't leak sensitive information in error messages
- Log security events appropriately
- Use structured logging for audit trails

## Release Process

### Version Management
- Follow semantic versioning (SemVer)
- Update all crate versions together
- Tag releases in git
- Generate changelogs

### Building Releases
```bash
# Build optimized release
cargo build --release

# Build Tauri app for distribution
pnpm tauri build

# Run full test suite
cargo test --release
```

### Documentation
- Update API documentation
- Regenerate documentation: `cargo doc --no-deps`
- Update user-facing documentation
- Create migration guides for breaking changes

## Troubleshooting

### Common Issues

#### Device Not Found
- Check device permissions (udev rules on Linux)
- Verify device is not in use by another application
- Check USB connection and device status

#### Compilation Errors
- Update Rust toolchain: `rustup update`
- Clean build directory: `cargo clean`
- Check platform-specific dependencies

#### FIDO2 Protocol Errors
- Verify device supports FIDO2
- Check PIN requirements
- Ensure proper user verification

### Getting Help
- Check existing GitHub issues
- Review documentation and examples
- Join the community discussion
- Contact maintainers for critical issues

---

This development guide provides comprehensive information for contributing to the YKey project. Follow these guidelines to ensure consistent, secure, and maintainable code. 