use std::process::Command;

use crate::{Protocol, SystemProxy};

fn networksetup() -> Command {
    Command::new("networksetup")
}

fn get_services() -> Vec<String> {
    let output = String::from_utf8(
        networksetup()
            .arg("-listallnetworkservices")
            .output()
            .expect("ERROR: Could not get network services")
            .stdout,
    )
    .expect("ERROR: Could not decode output to string");

    let services: Vec<&str> = output.split("\n").collect();

    let string_services: Vec<String> = services.iter().map(|s| s.to_string()).collect();

    string_services
}

fn set4protocol(proxy: &SystemProxy, service: String) {
    let string_protocol = match proxy.protocol {
        Protocol::HTTP => "webproxy".to_string(),
        Protocol::HTTPS => "securewebproxy".to_string(),
        Protocol::SOCKS => "socksfirewallproxy".to_string(),
        Protocol::ALL => "".to_string(),
    };

    let _ = networksetup()
        .args([
            format!("-set{}", string_protocol),
            service,
            proxy.host.clone(),
            proxy.port.to_string(),
        ])
        .output();
}

fn reset4protocol(proxy: &SystemProxy, service: String) {
    let string_protocol = match proxy.protocol {
        Protocol::HTTP => "webproxy".to_string(),
        Protocol::HTTPS => "securewebproxy".to_string(),
        Protocol::SOCKS => "socksfirewallproxy".to_string(),
        Protocol::ALL => "".to_string(),
    };

    let _ = networksetup()
        .args([
            format!("-set{}state", string_protocol),
            service,
            "off".to_string(),
        ])
        .output();
}

impl SystemProxy {
    pub fn get() -> SystemProxy {
        SystemProxy {
            is_enabled: true,
            host: "".to_string(),
            port: 61001,
            bypass: "".to_string(),
            protocol: Protocol::SOCKS,
        }
    }

    pub fn set(proxy: SystemProxy) {
        let services = get_services();

        for service in services {
            set4protocol(&proxy, service.to_string());
        }
    }

    pub fn unset(proxy: SystemProxy) {
        let services = get_services();

        for service in services {
            reset4protocol(&proxy, service.to_string());
        }
    }
}
