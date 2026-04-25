# 01 Log Bootstrap

## 职责

Log Bootstrap 在完整配置解码前建立最小日志能力，使配置错误、启动错误和 reload 错误都能按统一格式输出。

## 输入

- 用户配置文件路径。
- 用户配置 JSON 文本。

## 行为

1. 只读取根对象中的 `log` 配置。
2. `log` 缺失时使用 schema 默认值。
3. 只在该阶段初始化最小日志 sink，完整日志配置在 Compile Pipeline 后续阶段接管。
4. 反序列化错误直接使用 path-aware error 的 display 文本进入 `msg`。
5. text 格式日志中 `timestamp` 和 `level` 不带字段名。

## 输出

- `BootstrapLogConfig`
- `BootstrapLoggerGuard`

## 日志格式

```text
2026-04-24T12:00:00.000Z WARN event="config.decode" msg="log.level: unknown variant `warning`, expected one of `trace`, `debug`, `info`, `warn`, `error`" config_path="/etc/kitty/config.json"
```

## 错误处理

- `log` 配置反序列化失败时，仍然使用保底日志配置输出错误。
- `log` 缺失不是错误。
- 初始化日志输出失败时，启动失败。

## 测试要点

- `log` 缺失时能得到默认日志配置。
- `level` 非法时，错误信息包含配置路径和候选枚举值。
- text 格式符合 `timestamp level event msg fields` 顺序。
