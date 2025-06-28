# YKey Documentation

This directory contains comprehensive documentation for the YKey hardware security key management system.

## Documentation Structure

### 📋 [Architecture Overview](architecture.md)
Complete architectural documentation covering:
- System design and component relationships
- Crate structure and responsibilities
- Core patterns and design principles
- Data flow and state management
- Security considerations
- Future roadmap

### 📚 [API Reference](api-reference.md)
Detailed API documentation including:
- Core traits and their implementations
- Type definitions and data structures
- Error handling patterns
- Usage examples and best practices
- Platform integration points

### 🛠️ [Development Guide](development-guide.md)
Comprehensive guide for developers:
- Project setup and prerequisites
- Coding standards and conventions
- Feature development workflows
- Testing strategies and tools
- Debugging and troubleshooting
- Security guidelines

## Quick Start

### For Users
1. Download the latest release from the releases page
2. Install the application for your platform
3. Connect your hardware security keys
4. Follow the in-app setup wizard

### For Developers
1. Clone the repository
2. Install dependencies: `pnpm install`
3. Build the project: `cargo build`
4. Run development server: `pnpm tauri dev`
5. Read the [Development Guide](development-guide.md)

## Architecture at a Glance

YKey is built as a modular Rust workspace with a Tauri frontend:

```
┌─────────────────────────────────────────────────────────────┐
│                 React Frontend (TypeScript)                │
└─────────────────────┬───────────────────────────────────────┘
                      │ Tauri Bridge
┌─────────────────────▼───────────────────────────────────────┐
│                    Rust Backend                            │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │ ykey-core   │ │ykey-protocol│ │ykey-platform│          │
│  │ (traits)    │ │ (FIDO2)     │ │(OS-specific)│          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
│                 ┌─────────────┐                           │
│                 │ykey-device  │                           │
│                 │(management) │                           │
│                 └─────────────┘                           │
└─────────────────────────────────────────────────────────────┘
```

## Key Features

### Device Support
- **YubiKey**: Full support for YubiKey 4, 5, and Bio series
- **CanoKey**: Support for CanoKey Pigeon and other variants
- **Nitrokey**: Nitrokey 3 and FIDO2 models
- **SoloKey**: Solo 1 and Solo 2 devices
- **Generic**: Any FIDO2-compliant security key

### Protocol Support
- **FIDO2/WebAuthn**: Complete implementation with PIN and biometric support
- **CTAP 1.0/2.0**: Client-to-Authenticator Protocol
- **OATH** *(planned)*: TOTP/HOTP for OTP generation
- **PIV** *(planned)*: Personal Identity Verification
- **OpenPGP** *(planned)*: OpenPGP smart card functionality

### Platform Support
- **Desktop**: Windows, macOS, Linux via Tauri
- **Mobile** *(planned)*: Android and iOS with NFC support
- **Cross-platform**: Unified codebase with platform-specific optimizations

## Security Model

### Memory Safety
- Written in Rust for memory safety guarantees
- No unsafe code in core business logic
- Careful handling of cryptographic material

### Communication Security
- Direct hardware communication without network exposure
- Secure PIN and biometric data handling
- Proper cleanup of sensitive information

### Dependency Security
- Minimal external dependencies
- Well-audited cryptographic libraries
- Regular security updates

## Contributing

We welcome contributions! Please:

1. Read the [Development Guide](development-guide.md)
2. Check existing issues and discussions
3. Follow our coding standards
4. Submit pull requests with comprehensive tests
5. Update documentation as needed

### Areas for Contribution
- **Device Support**: Add support for new hardware devices
- **Protocols**: Implement additional security protocols
- **Platforms**: Extend platform support
- **UI/UX**: Improve user interface and experience
- **Documentation**: Enhance guides and examples
- **Testing**: Add tests and improve coverage

## Project Status

### Current Phase: Alpha Development
- ✅ Core architecture implemented
- ✅ Basic FIDO2 support
- ✅ Device discovery and management
- ✅ Cross-platform foundation
- 🚧 UI implementation in progress
- 🚧 Protocol implementations ongoing
- 📋 Additional device support planned

### Roadmap
- **v0.1.0**: Basic FIDO2 device management
- **v0.2.0**: OATH and PIV protocol support
- **v0.3.0**: Mobile platform support
- **v1.0.0**: Production-ready release

## Support and Community

### Getting Help
- **Documentation**: Start with this documentation
- **Issues**: Report bugs and request features on GitHub
- **Discussions**: Join community discussions for questions
- **Security**: Report security issues privately to maintainers

### Resources
- [Rust Programming Language](https://www.rust-lang.org/)
- [Tauri Framework](https://tauri.app/)
- [FIDO Alliance Specifications](https://fidoalliance.org/specifications/)
- [WebAuthn Guide](https://webauthn.guide/)

## License

This project is licensed under [LICENSE] - see the license file for details.

## Acknowledgments

- FIDO Alliance for security standards
- Rust community for excellent libraries
- Tauri team for the cross-platform framework
- Hardware security key manufacturers for device support

---

**Note**: This project is under active development. APIs and interfaces may change before the stable release. Please check the latest documentation and releases for current status. 