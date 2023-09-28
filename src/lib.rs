#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

pub enum Protocol {
    HTTP,
    HTTPS,
    SOCKS,
    ALL,
}
pub struct SystemProxy {
    pub is_enabled: bool,
    pub protocol: Protocol,
    pub host: String,
    pub port: u16,
    pub bypass: String,
}
