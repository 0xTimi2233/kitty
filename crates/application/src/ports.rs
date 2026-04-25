//! 控制面端口定义。
//!
//! application 通过端口调用 infrastructure，避免平台细节进入领域层。

/// 配置读取端口。
pub trait ConfigSource {}

/// 运行态发布端口。
pub trait RuntimePublisher {}
