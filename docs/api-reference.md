# API Reference Documentation

## Core Traits

### Device Trait

The `Device` trait provides the fundamental interface for hardware key communication.

```rust
#[async_trait]
pub trait Device: Send + Sync {
    /// Get device information including capabilities and metadata
    async fn info(&self) -> XKeyResult<DeviceInfo>;
    
    /// Establish connection to the hardware device
    async fn connect(&mut self) -> XKeyResult<()>;
    
    /// Disconnect from the hardware device
    async fn disconnect(&mut self) -> XKeyResult<()>;
    
    /// Check if device is currently connected
    fn is_connected(&self) -> bool;
    
    /// Send raw bytes to device and receive response
    async fn send_raw(&mut self, data: &[u8]) -> XKeyResult<Vec<u8>>;
}
```

### Fido2Protocol Trait

The `Fido2Protocol` trait implements FIDO2/WebAuthn operations.

```rust
#[async_trait]
pub trait Fido2Protocol: Send + Sync {
    /// Get authenticator information and capabilities
    async fn get_info(&mut self) -> XKeyResult<AuthenticatorInfo>;
    
    /// Create a new credential (registration)
    async fn make_credential(
        &mut self, 
        params: MakeCredentialParams
    ) -> XKeyResult<AttestationObject>;
    
    /// Get assertion for authentication
    async fn get_assertion(
        &mut self, 
        params: GetAssertionParams
    ) -> XKeyResult<AssertionObject>;
    
    /// Reset the authenticator (factory reset)
    async fn reset(&mut self) -> XKeyResult<()>;
    
    /// Set PIN for the authenticator
    async fn set_pin(&mut self, pin: &str) -> XKeyResult<()>;
    
    /// Change existing PIN
    async fn change_pin(&mut self, old_pin: &str, new_pin: &str) -> XKeyResult<()>;
    
    /// Verify PIN
    async fn verify_pin(&mut self, pin: &str) -> XKeyResult<bool>;
}
```

### DeviceDiscovery Trait

The `DeviceDiscovery` trait handles device enumeration and monitoring.

```rust
#[async_trait]
pub trait DeviceDiscovery: Send + Sync {
    /// Scan for available devices
    async fn scan(&self) -> XKeyResult<Vec<DeviceInfo>>;
    
    /// Watch for device connection/disconnection events
    async fn watch(&self) -> XKeyResult<DeviceEventStream>;
}
```

### DeviceCreator Trait

The `DeviceCreator` trait is used by the factory pattern for device instantiation.

```rust
pub trait DeviceCreator: Send + Sync {
    /// Create a device instance from device info
    fn create(&self, info: &DeviceInfo) -> XKeyResult<Box<dyn Device>>;
    
    /// Check if this creator supports the given device
    fn supports(&self, info: &DeviceInfo) -> bool;
}
```

### CredentialStore Trait

The `CredentialStore` trait provides credential persistence.

```rust
#[async_trait]
pub trait CredentialStore: Send + Sync {
    /// Store a credential
    async fn store(&mut self, credential: &Credential) -> XKeyResult<()>;
    
    /// Retrieve a credential by ID
    async fn get(&self, id: &CredentialId) -> XKeyResult<Option<Credential>>;
    
    /// List all stored credentials
    async fn list(&self) -> XKeyResult<Vec<Credential>>;
    
    /// Delete a credential by ID
    async fn delete(&mut self, id: &CredentialId) -> XKeyResult<()>;
}
```

## Core Types

### DeviceInfo

Contains metadata about a discovered hardware device.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,                           // Unique device identifier
    pub name: String,                         // Human-readable device name
    pub manufacturer: String,                 // Device manufacturer
    pub product_name: String,                 // Product name
    pub serial_number: Option<String>,        // Serial number (if available)
    pub vendor_id: u16,                       // USB vendor ID
    pub product_id: u16,                      // USB product ID
    pub device_type: DeviceType,              // Type classification
    pub transport: TransportType,             // Communication transport
    pub capabilities: Vec<Capability>,        // Supported capabilities
    pub firmware_version: Option<String>,     // Firmware version (if available)
}
```

### DeviceType

Enumeration of supported device types.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    YubiKey,        // Yubico YubiKey devices
    CanoKey,        // CanoKeys devices
    Nitrokey,       // Nitrokey devices
    SoloKey,        // SoloKeys devices
    Generic,        // Generic FIDO2 devices
}
```

### TransportType

Communication transport methods.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportType {
    Usb,            // USB HID transport
    Nfc,            // Near Field Communication
    Bluetooth,      // Bluetooth transport
    Hybrid,         // Hybrid transport (multiple methods)
}
```

### Capability

Device capabilities enumeration.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    Fido2,          // FIDO2/WebAuthn support
    Fido1,          // FIDO U2F support
    Oath,           // OATH TOTP/HOTP support
    Piv,            // PIV (Personal Identity Verification)
    OpenPgp,        // OpenPGP support
    Otp,            // OTP (One-Time Password) support
}
```

### Credential

Represents a stored credential.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: CredentialId,                                    // Credential identifier
    pub rp_id: String,                                       // Relying party identifier
    pub user_id: Vec<u8>,                                    // User identifier
    pub user_name: String,                                   // User name
    pub user_display_name: String,                           // User display name
    pub public_key: Vec<u8>,                                 // Public key bytes
    pub counter: u32,                                        // Usage counter
    pub created_at: chrono::DateTime<chrono::Utc>,           // Creation timestamp
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,    // Last usage timestamp
}

pub type CredentialId = Vec<u8>;
```

### FIDO2 Parameter Types

#### MakeCredentialParams

Parameters for creating a new credential.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeCredentialParams {
    pub client_data_hash: Vec<u8>,                           // Hash of client data
    pub rp: RelyingParty,                                    // Relying party info
    pub user: User,                                          // User info
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameter>, // Key parameters
    pub exclude_list: Option<Vec<PublicKeyCredentialDescriptor>>, // Excluded credentials
    pub extensions: Option<HashMap<String, serde_json::Value>>,   // Extensions
    pub options: MakeCredentialOptions,                      // Creation options
    pub pin_uv_auth_param: Option<Vec<u8>>,                 // PIN/UV auth parameter
    pub pin_uv_auth_protocol: Option<u8>,                   // PIN/UV auth protocol
}
```

#### GetAssertionParams

Parameters for getting an assertion (authentication).

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAssertionParams {
    pub rp_id: String,                                       // Relying party ID
    pub client_data_hash: Vec<u8>,                          // Hash of client data
    pub allow_list: Option<Vec<PublicKeyCredentialDescriptor>>, // Allowed credentials
    pub extensions: Option<HashMap<String, serde_json::Value>>,  // Extensions
    pub options: GetAssertionOptions,                        // Assertion options
    pub pin_uv_auth_param: Option<Vec<u8>>,                 // PIN/UV auth parameter
    pub pin_uv_auth_protocol: Option<u8>,                   // PIN/UV auth protocol
}
```

## Core Structures

### DeviceFactory

Factory for creating device instances.

```rust
pub struct DeviceFactory {
    creators: HashMap<DeviceType, Box<dyn DeviceCreator>>,
}

impl DeviceFactory {
    /// Create a new factory with default creators
    pub fn new() -> Self;
    
    /// Register a device creator for a specific type
    pub fn register(&mut self, device_type: DeviceType, creator: Box<dyn DeviceCreator>);
    
    /// Create a device instance from device info
    pub fn create_device(&self, info: &DeviceInfo) -> XKeyResult<Box<dyn Device>>;
}
```

### DeviceManager

Manages device connections and lifecycle.

```rust
pub struct DeviceManager {
    factory: Arc<DeviceFactory>,
    discoveries: Vec<Box<dyn DeviceDiscovery>>,
    connected_devices: Arc<RwLock<HashMap<String, Box<dyn Device>>>>,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> Self;
    
    /// Add a device discovery mechanism
    pub fn add_discovery(&mut self, discovery: Box<dyn DeviceDiscovery>);
    
    /// Scan for available devices
    pub async fn scan_devices(&self) -> XKeyResult<Vec<DeviceInfo>>;
    
    /// Connect to a specific device
    pub async fn connect_device(&self, device_id: &str) -> XKeyResult<()>;
    
    /// Get a connected device reference
    pub async fn get_device(&self, device_id: &str) -> XKeyResult<Option<&dyn Device>>;
}
```

### Fido2Client

FIDO2 protocol client implementation.

```rust
pub struct Fido2Client<D: Device> {
    device: D,
    pin_token: Option<Vec<u8>>,
}

impl<D: Device> Fido2Client<D> {
    /// Create a new FIDO2 client with a device
    pub fn new(device: D) -> Self;
    
    /// Send a CTAP command to the device
    async fn send_ctap_command(&mut self, command: CtapCommand) -> XKeyResult<CtapResponse>;
}
```

## Error Types

### XKeyResult

Type alias for Results in the XKey ecosystem.

```rust
pub type XKeyResult<T> = Result<T, XKeyError>;
```

### XKeyError

Comprehensive error enumeration for the system.

```rust
#[derive(Debug, thiserror::Error)]
pub enum XKeyError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    
    #[error("Unsupported device type: {0:?}")]
    UnsupportedDevice(DeviceType),
    
    #[error("Device communication error")]
    CommunicationError,
    
    #[error("CTAP error: {0}")]
    CtapError(u8),
    
    #[error("Unexpected response from device")]
    UnexpectedResponse,
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Operation cancelled by user")]
    UserCancelled,
    
    #[error("Invalid PIN")]
    InvalidPin,
    
    #[error("Device is locked")]
    DeviceLocked,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

## Platform Integration

### Platform Discovery Factory

```rust
/// Create platform-specific device discovery
pub fn create_platform_discovery() -> Box<dyn DeviceDiscovery>;
```

This function returns the appropriate device discovery implementation for the current platform:
- Windows: HID API-based discovery
- macOS: IOKit-based discovery  
- Linux: hidraw/libusb-based discovery
- Android: NFC-based discovery (planned)
- iOS: NFC-based discovery (planned)

## Usage Examples

### Basic Device Connection

```rust
use ykey_device::DeviceManager;
use ykey_platform::create_platform_discovery;

async fn connect_first_device() -> XKeyResult<()> {
    let mut manager = DeviceManager::new();
    manager.add_discovery(create_platform_discovery());
    
    let devices = manager.scan_devices().await?;
    if let Some(device) = devices.first() {
        manager.connect_device(&device.id).await?;
        println!("Connected to device: {}", device.name);
    }
    
    Ok(())
}
```

### FIDO2 Authentication

```rust
use ykey_protocol::Fido2Client;

async fn authenticate(device: impl Device, params: GetAssertionParams) -> XKeyResult<AssertionObject> {
    let mut client = Fido2Client::new(device);
    client.get_assertion(params).await
}
```

---

This API reference provides the foundation for working with the YKey hardware security key management system. 