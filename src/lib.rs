#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

#[derive(Debug)]
pub enum Protocol {
    HTTP,
    HTTPS,
    SOCKS,
    ALL,
}

/// # Example (set):
///
/// ```
/// use rustem_proxy::SystemProxy;
/// SystemProxy::set(SystemProxy {
///      is_enabled: true,
///      host: "127.0.0.1".to_string(),
///      port: 61001,
///      bypass: "".to_string(),
///      protocol: rustem_proxy::Protocol::HTTP,
/// });
/// ```
///
///  # Example (get):
///
/// ```
/// use rustem_proxy::SystemProxy;
///
/// let proxy = SystemProxy::get();
/// println!("Is enabled: {}", proxy.is_enabled);
/// println!("host: {}", proxy.host);
/// println!("port: {}", proxy.port);
/// println!("protocol: {:?}", proxy.protocol);
/// ```
pub struct SystemProxy {
    pub is_enabled: bool,
    pub protocol: Protocol,
    pub host: String,
    pub port: u16,
    pub bypass: String,
}
