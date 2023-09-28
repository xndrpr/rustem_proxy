# rustem_proxy

## Usage

Example:

```rust
use rustem_proxy::SystemProxy;

SystemProxy::set(SystemProxy {
    is_enabled: true,
    host: "127.0.0.1".to_string(),
    port: 61001,
    bypass: "".to_string(),
    protocol: rustem_proxy::Protocol::HTTP,
});
```
