# 05 Basic Validate

## 职责

Basic Validate 检查单配置对象和简单集合约束，确保后续语义阶段可以安全处理引用关系。

## 输入

- `NormalizedConfig`

## 输出

- `BasicValidConfig`

## 校验内容

### tag

- DNS server tag 不允许重复。
- inbound tag 不允许重复。
- outbound tag 不允许重复。
- rule_set tag 不允许重复。
- tag 不能为空。

### remote rule_set

- URL 必须使用 `https`。
- 即使未被引用，也必须完成 schema 和 basic validate。
- `update_interval` 启用时必须大于等于 `15m`。

### local rule_set

- path 不能为空。
- path normalize 后必须可用于后续读取。

### action

- DNS action 只允许 schema 定义的 DNS action 类型。
- Route action 只允许 schema 定义的 Route action 类型。
- action 快捷写法必须已经在 Normalize 阶段转换为对象级 action。

## 非职责

- 不校验 action target 是否存在。
- 不校验 resolver 是否形成回环。
- 不读取 rule_set 文件内容。
- 不下载 remote rule_set。

## 测试要点

- tag 重复会失败。
- remote rule_set 使用非 https URL 会失败。
- `update_interval = 0` 通过校验并表示禁用。
- `update_interval = 10m` 失败。
