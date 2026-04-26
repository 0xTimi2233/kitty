# 日志配置

日志配置字段：

- level：debug、info、warn、error。
- format：text、json。
- timestamp：bool。

text 日志格式：

```text
2026-04-24T12:00:00.000Z WARN event="config.decode" msg="log.level: unknown variant `warning`, expected one of `debug`, `info`, `warn`, `error`" config_path="/etc/kitty/config.json"
```

反序列化错误使用带路径的 display 文本进入 `msg`。非反序列化阶段需要维护可定位的配置路径。
