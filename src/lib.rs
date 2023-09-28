#[cfg(target_os = "windows")]
mod windows;

pub struct SystemProxy {
    pub is_enabled: bool,
    pub host: String,
    pub port: u16,
    pub bypass: String,
}
