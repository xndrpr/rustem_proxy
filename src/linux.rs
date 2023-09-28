use std::process::Command;

use crate::{Protocol, SystemProxy};

const CMD_KEY: &str = "org.gnome.system.proxy";

fn gsettings() -> Command {
    Command::new("gsettings")
}

impl SystemProxy {
    pub fn get() -> SystemProxy {
        // TODO
        SystemProxy {
            is_enabled: true,
            host: "".to_string(),
            port: 61001,
            bypass: "".to_string(),
            protocol: Protocol::SOCKS,
        }
    }

    pub fn set(proxy: SystemProxy) {
        let service = match proxy.protocol {
            Protocol::HTTP => "http".to_string(),
            Protocol::HTTPS => "https".to_string(),
            Protocol::SOCKS => "socks".to_string(),
            Protocol::ALL => "".to_string(),
        };
        let schema = format!("{CMD_KEY}.{service}");
        let schema = schema.as_str();

        let host = format!("'{}'", proxy.host);
        let host = host.as_str();
        let port = format!("{}", proxy.port);
        let port = port.as_str();

        gsettings()
            .args(["set", schema, "host", host])
            .status()
            .expect("ERROR: Could not set host");
        gsettings()
            .args(["set", schema, "port", port])
            .status()
            .expect("ERROR: Could not set port");
    }
}
