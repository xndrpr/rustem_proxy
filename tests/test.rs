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
        });

        assert!(true);
    }
}
