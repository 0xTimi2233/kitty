use std::time::Duration;

use kitty_acl::helper::duration::RefreshInterval;
use kitty_acl::schema::ConfigRoot;
use kitty_acl::schema::common::rule::DnsStrategy;

#[test]
fn decode_defaults() {
    let json = r#"{}"#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    assert!(config.log.timestamp);
    assert_eq!(config.dns.cache.capacity, 4096);
}

#[test]
fn normalize_domain_and_tag() {
    let json = r#"
    {
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
fn update_interval_zero_is_disabled() {
    let json = r#"
    {
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
    let json = r#"{ "dns": { "strategy": "prefer_ipv6" } }"#;
    let config: ConfigRoot = serde_json::from_str(json).expect("decode config");
    assert_eq!(config.dns.strategy, Some(DnsStrategy::PreferIpv6));
}
