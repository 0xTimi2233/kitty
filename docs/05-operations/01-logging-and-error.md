# 日志与错误

日志格式：

```text
timestamp level event="..." msg="..." field="..."
```

要求：

- event 使用固定枚举。
- msg 使用 display 输出。
- 反序列化错误 display 文本进入 msg。
- 非反序列化阶段维护稳定 json path。
- 日志和错误文本使用英文。

示例：

```text
2026-04-24T12:00:00.000Z WARN event="config.decode" msg="log.level: unknown variant `warning`, expected one of `trace`, `debug`, `info`, `warn`, `error`" config_path="/etc/kitty/config.json"
```
