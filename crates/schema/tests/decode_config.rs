use kitty_schema::Config;

#[test]
fn decode_minimal_config() {
    let raw = r#"
    {
      "log": { "level": "warn", "format": "text" },
      "dns": {
        "servers": [
          { "type": "udp", "tag": "local", "server": "1.1.1.1" },
          { "type": "https", "tag": "doh", "server": "dns.example.com" }
        ],
        "rules": [
          { "domain_suffix": ".example.com", "server": "local" }
        ]
      },
      "outbounds": [
        { "type": "direct", "tag": "direct" }
      ],
      "route": {
        "rules": [
          { "domain": "example.com", "outbound": "direct" }
        ]
      }
    }
    "#;

    let decoded: Result<Config, _> = serde_json::from_str(raw);
    assert!(decoded.is_ok());
}

#[test]
fn rule_set_update_interval_zero_is_valid() {
    let raw = r#"
    {
      "route": {
        "rule_set": [
          {
            "type": "remote",
            "tag": "remote-a",
            "format": "source",
            "url": "https://example.com/rules.json",
            "update_interval": "0s"
          }
        ]
      }
    }
    "#;

    let decoded: Result<Config, _> = serde_json::from_str(raw);
    assert!(decoded.is_ok());
}
