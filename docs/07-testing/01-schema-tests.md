# 01 Schema Tests

## 目标

验证 schema crate 可以稳定解码用户配置，并正确填充默认值、执行字符串 normalize 和输入形态归一化。

## 测试范围

### Decode

- root 配置完整解码。
- 必填字段缺失时报错。
- 未知字段时报错。
- enum 非法值时报错。

### Default values

- log 缺失时使用默认日志配置。
- DNS cache 字段缺失时填充默认值。
- rule_set `update_interval` 缺失时为 `1d`。
- bool 默认 false 的字段使用缺省值。

### update_interval

- `0` 解码为 disabled。
- `"0s"` 解码为 disabled。
- `"1d"` 解码为 enabled interval。

### String normalize

- tag：trim + lowercase。
- domain：trim + lowercase。
- domain_suffix：trim + lowercase + 去前导点。
- path：trim，不 lowercase。

### one-or-many

- 单值输入转为数组。
- 数组输入保持数组。
- null 或缺失转为空数组。

## 输出要求

测试失败信息必须能定位字段和输入片段。

## 推荐测试数据

- 最小合法配置。
- 完整合法配置。
- 包含混合大小写和空白字符的配置。
- 包含单值和数组混合输入的配置。
