// crates/xkey-platform/src/lib.rs
pub mod discovery;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

// 统一平台接口
pub fn create_platform_discovery() -> Box<dyn xkey_core::traits::DeviceDiscovery> {
    #[cfg(target_os = "windows")]
    return Box::new(windows::WindowsHidDiscovery::new());
    
    #[cfg(target_os = "macos")]
    return Box::new(macos::MacOsHidDiscovery::new());
    
    #[cfg(target_os = "linux")]
    return Box::new(linux::LinuxHidDiscovery::new());
    
    #[cfg(target_os = "android")]
    return Box::new(android::AndroidNfcDiscovery::new());
    
    #[cfg(target_os = "ios")]
    return Box::new(ios::IosNfcDiscovery::new());
    
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos", 
        target_os = "linux",
        target_os = "android",
        target_os = "ios"
    )))]
    compile_error!("Unsupported platform");
}
