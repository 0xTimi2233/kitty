use std::time::Duration;

use kitty_acl::helper::duration::RefreshInterval;
use kitty_acl::schema::ConfigRoot;
use kitty_acl::schema::common::rule::DnsStrategy;
use kitty_acl::schema::common::rule::VlessPacketEncoding;

#[test]
fn decode_missing_log_uses_default() {
    let json = r#"
    {
      "dns": { "servers": [] },
      "inbounds": [],
      "outbounds": [],
      "route": {}
    }
    "#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    assert_eq!(config.log.level, kitty_acl::schema::log::LogLevel::Warn);
    assert_eq!(config.log.format, kitty_acl::schema::log::LogFormat::Text);
    assert!(config.log.timestamp);
    assert_eq!(config.dns.cache.capacity, 4096);
}

#[test]
fn decode_missing_required_root_field_fails() {
    let json = r#"
    {
      "inbounds": [],
      "outbounds": [],
      "route": {}
    }
    "#;
    let error = serde_json::from_str::<ConfigRoot>(json).expect_err("missing dns should fail");
    assert!(error.to_string().contains("missing field `dns`"));
}

#[test]
fn normalize_domain_and_tag() {
    let json = r#"
    {
      "inbounds": [],
      "outbounds": [],
      "route": {},
      "dns": {
        "servers": [
          { "type": "udp", "tag": " Main ", "server": " DNS.EXAMPLE.COM " }
        ],
        "rules": [
          { "domain_suffix": ".Example.COM", "server": " Main " }
        ]
      }
    }
    "#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    match &config.dns.servers[0] {
        kitty_acl::schema::dns::server::DnsServer::Udp(server) => {
            assert_eq!(server.tag, "main");
            assert_eq!(server.server, "dns.example.com");
        }
        _ => panic!("unexpected server"),
    }
}

#[test]
fn decode_missing_dns_servers_fails() {
    let json = r#"
    {
      "dns": {},
      "inbounds": [],
      "outbounds": [],
      "route": {}
    }
    "#;
    let error =
        serde_json::from_str::<ConfigRoot>(json).expect_err("missing dns.servers should fail");
    assert!(error.to_string().contains("missing field `servers`"));
}

#[test]
fn update_interval_zero_is_disabled() {
    let json = r#"
    {
      "dns": { "servers": [] },
      "inbounds": [],
      "outbounds": [],
      "route": {
        "rule_set": [
          { "type": "remote", "tag": "r", "format": "source", "url": "https://example.com/r.json", "update_interval": "0s" }
        ]
      }
    }
    "#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    let kitty_acl::schema::rule_set::RuleSet::Remote(remote) = &config.route.rule_set[0] else {
        panic!("unexpected rule set")
    };
    assert_eq!(remote.update_interval, RefreshInterval::Disabled);
}

#[test]
fn update_interval_default_is_one_day() {
    let json = r#"
    {
      "dns": { "servers": [] },
      "inbounds": [],
      "outbounds": [],
      "route": {
        "rule_set": [
          { "type": "remote", "tag": "r", "format": "source", "url": "https://example.com/r.json" }
        ]
      }
    }
    "#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    let kitty_acl::schema::rule_set::RuleSet::Remote(remote) = &config.route.rule_set[0] else {
        panic!("unexpected rule set")
    };
    assert_eq!(
        remote.update_interval,
        RefreshInterval::Interval(Duration::from_secs(86_400))
    );
}

#[test]
fn dns_strategy_is_preserved() {
    let json = r#"
    {
      "dns": {
        "servers": [],
        "strategy": "prefer_ipv6"
      },
      "inbounds": [],
      "outbounds": [],
      "route": {}
    }
    "#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    assert_eq!(config.dns.strategy, Some(DnsStrategy::PreferIpv6));
}

#[test]
fn log_fields_use_field_level_defaults() {
    let json = r#"
    {
      "log": { "timestamp": false },
      "dns": { "servers": [] },
      "inbounds": [],
      "outbounds": [],
      "route": {}
    }
    "#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    assert_eq!(config.log.level, kitty_acl::schema::log::LogLevel::Warn);
    assert_eq!(config.log.format, kitty_acl::schema::log::LogFormat::Text);
    assert!(!config.log.timestamp);
}

#[test]
fn dns_cache_nested_objects_use_defaults() {
    let json = r#"
    {
      "dns": {
        "servers": [],
        "cache": {
          "lazy_cache": {},
          "dump": {}
        }
      },
      "inbounds": [],
      "outbounds": [],
      "route": {}
    }
    "#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    assert!(config.dns.cache.enable);
    assert_eq!(config.dns.cache.capacity, 4096);
    assert!(!config.dns.cache.lazy_cache.enable);
    assert_eq!(config.dns.cache.lazy_cache.ttl, 86_400);
    assert_eq!(config.dns.cache.lazy_cache.reply_ttl, 5);
    assert!(!config.dns.cache.dump.enable);
    assert_eq!(config.dns.cache.dump.path, ".cache/dump.db");
    assert_eq!(config.dns.cache.dump.interval, 3_600);
}

#[test]
fn vless_packet_encoding_uses_enum_default() {
    let json = r#"
    {
      "dns": { "servers": [] },
      "inbounds": [],
      "outbounds": [
        {
          "type": "vless",
          "tag": "proxy",
          "server": "example.com",
          "server_port": 443,
          "uuid": "11111111-1111-1111-1111-111111111111"
        }
      ],
      "route": {}
    }
    "#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    let kitty_acl::schema::outbound::Outbound::Vless(outbound) = &config.outbounds[0] else {
        panic!("unexpected outbound")
    };
    assert_eq!(outbound.packet_encoding, VlessPacketEncoding::Xudp);
}
