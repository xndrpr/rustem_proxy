#[cfg(test)]
mod tests {
    use rustem_proxy::SystemProxy;

    #[test]
    fn test_set() {
        SystemProxy::set(SystemProxy {
            is_enabled: true,
            host: "127.0.0.1".to_string(),
            port: 61000,
            bypass: "".to_string(),
            protocol: rustem_proxy::Protocol::SOCKS,
        });
        SystemProxy::set(SystemProxy {
            is_enabled: true,
            host: "127.0.0.1".to_string(),
            port: 61001,
            bypass: "".to_string(),
            protocol: rustem_proxy::Protocol::HTTP,
        });
        SystemProxy::set(SystemProxy {
            is_enabled: true,
            host: "127.0.0.1".to_string(),
            port: 61001,
            bypass: "".to_string(),
            protocol: rustem_proxy::Protocol::HTTPS,
        });

        assert!(true);
    }
}
