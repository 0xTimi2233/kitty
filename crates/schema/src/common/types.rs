//! schema 公共类型。

use serde::{Deserialize, Serialize};

/// 端口范围原始表示。
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PortRange(pub String);

/// IP CIDR 原始表示。
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IpCidr(pub String);
