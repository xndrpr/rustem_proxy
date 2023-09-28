use crate::SystemProxy;
use std::{net::SocketAddr, str::FromStr};
use winapi::shared::ntdef::NULL;
use winapi::um::wininet::{
    InternetSetOptionA, INTERNET_OPTION_REFRESH, INTERNET_OPTION_SETTINGS_CHANGED,
};
use winreg::{enums, RegKey};

const SUB_KEY: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";

impl SystemProxy {
    pub fn get() -> SystemProxy {
        let current_user = RegKey::predef(enums::HKEY_CURRENT_USER);
        let key = current_user
            .open_subkey_with_flags(SUB_KEY, enums::KEY_READ)
            .map_err(|e| eprintln!("ERROR: Could not open subkey with flags: {}", e.to_string()))
            .unwrap();
        let is_enabled = key
            .get_value::<u32, _>("ProxyEnable")
            .expect("ERROR: Could not get key value")
            == 1u32;
        let server = key
            .get_value::<String, _>("ProxyServer")
            .expect("ERROR: Could not get key value");
        let server = server.as_str();
        let socket = SocketAddr::from_str(server)
            .map_err(|e| {
                eprintln!("ERROR: Could not parse from str: {}", e.to_string());
            })
            .unwrap();
        let host = socket.ip().to_string();
        let port = socket.port();
        let bypass: String = key.get_value("ProxyOverride").unwrap_or("".into());

        SystemProxy {
            is_enabled,
            host: host,
            port: port,
            bypass: bypass,
        }
    }

    pub fn set(proxy: SystemProxy) {
        let hkcu = RegKey::predef(enums::HKEY_CURRENT_USER);
        let cur_var = hkcu
            .open_subkey_with_flags(SUB_KEY, enums::KEY_SET_VALUE)
            .expect("ERROR: Could not open subkey with flags");

        let enable = if proxy.is_enabled { 1u32 } else { 0u32 };
        let server = format!("{}:{}", proxy.host, proxy.port);
        let bypass = proxy.bypass.as_str();

        cur_var
            .set_value("ProxyEnable", &enable)
            .expect("ERROR: Could not set value ProxyEnable");
        cur_var
            .set_value("ProxyServer", &server)
            .expect("ERROR: Could not set value ProxyServer");
        cur_var
            .set_value("ProxyOverride", &bypass)
            .expect("ERROR: Could not set value ProxyOverride");

        unsafe {
            InternetSetOptionA(NULL, INTERNET_OPTION_SETTINGS_CHANGED, NULL, 0);
            InternetSetOptionA(NULL, INTERNET_OPTION_REFRESH, NULL, 0);
        }
    }
}
