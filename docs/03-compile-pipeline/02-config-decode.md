# 02 Config Decode

## 职责

Config Decode 将用户 JSON 文本反序列化为 `schema` crate 中定义的配置结构。

该阶段只处理输入格式、默认值填充和 serde 注解，不处理跨字段校验、不解析引用、不下载 rule_set。

## 输入

- 配置文件路径。
- 配置 JSON 文本。
- Bootstrap logger。

## 输出

- `SchemaRoot`
- `ConfigSourceInfo`

## 行为

1. 使用 path-aware decode 记录反序列化错误位置。
2. schema 中有默认值的字段在本阶段完成默认值填充。
3. schema 中必填字段缺失时直接反序列化失败。
4. 支持单值或数组输入的字段在本阶段统一成数组。
5. `update_interval` 缺失时填充为 `1d`；`0` 和 `"0s"` 统一表示禁用自动刷新。

## 非职责

- 不做 tag 唯一性校验。
- 不做 action target 校验。
- 不做 rule_set 下载。
- 不做 rule_set 展开。
- 不做 Runtime Model 构建。

## 错误处理

反序列化失败时输出：

- `event="config.decode"`
- `msg` 为 path-aware error 的 display 文本。
- `config_path` 为配置文件路径。

## 测试要点

- 缺失默认字段可以解码成功。
- 缺失必填字段会失败。
- 单值和数组输入能得到同一内部结构。
- 错误信息中包含字段路径。
