# DNS 配置

DNS 配置包含：

- servers：tcp、udp、tls、quic、https、h3。
- rules：leaf rule 或 logical rule。
- final：默认 DNS server 引用。
- strategy：DNS 查询策略。
- cache：DNS cache、lazy cache、dump。

DNS rule action 支持：

- route。
- reject。
- predefined。

快捷 `server` 写法由 structural normalize 转成对象 action。
