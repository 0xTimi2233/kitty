# DDD 边界

Kitty 使用非严格 DDD，核心目标是防腐和可维护性。

## 管理面

管理面包含：

- CLI 入口。
- pid lock。
- 系统信号。
- start / reload 触发。

## 控制面

控制面包含：

- 配置解码。
- normalize。
- 校验。
- rule_set 加载。
- 规则展开合并。
- 索引编译。
- runtime 发布。

## 数据面

数据面包含：

- DNS 入站。
- Proxy 入站。
- 匹配器。
- action dispatch。
- outbound 执行。
- eBPF fast path。

## 防腐原则

- `schema` 不进入数据面。
- 外部协议库只出现在 `infrastructure`。
- eBPF 用户态 loader 与 eBPF program 分离。
- runtime model 发布后只读。
