# 03 Schema Normalize

## 职责

Schema Normalize 将反序列化后的配置统一为后续编译阶段可处理的规范形态。

Normalize 不是语义校验。它只处理输入写法差异、大小写、空白字符、快捷写法和规则 AST 形态。

## 输入

- `SchemaRoot`

## 输出

- `NormalizedConfig`

## 规范化规则

### 字符串

- 普通 tag 和枚举字符串：`trim + lowercase`。
- path 类字段：只执行 `trim`。
- domain、domain_suffix、domain_keyword：`trim + lowercase`。
- domain_suffix：去掉前导点。
- domain_regex：不修改大小写。

### action

兼容快捷写法，但 Normalize 后统一为对象级 action。

示例：

```json
{ "outbound": "proxy" }
```

Normalize 后等价于：

```json
{ "action": { "type": "route", "outbound": "proxy" } }
```

DNS server 快捷写法同理，Normalize 后统一为对象级 action。

### rule

- 默认规则和逻辑规则都转换为统一 Rule IR。
- Rule IR 只包含 `and`、`or`、`not`、`leaf` 这几类关系。
- rule_set 引用不在本阶段展开，只保留引用关系。

### update_interval

- 缺失：`Interval(1d)`。
- `0`：`Disabled`。
- `"0s"`：`Disabled`。
- 其他 duration：`Interval(duration)`。

## 非职责

- 不校验 tag 是否存在。
- 不校验 rule_set 是否可下载。
- 不检测回环。
- 不构建索引。

## 测试要点

- 字符串 normalize 不产生无意义分配。
- domain_suffix 能去掉前导点。
- action 快捷写法能转换为对象级 action。
- DefaultRule 和 LogicalRule 能转换为统一 Rule IR。
