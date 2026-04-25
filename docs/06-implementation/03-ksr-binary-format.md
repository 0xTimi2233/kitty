# KSR 二进制格式

KSR 是内部 rule set binary 格式，后缀为 `.ksr`。

目标：

- 快速加载。
- 固定 header。
- 支持版本识别。
- 支持 hash 校验。
- 避免启动时重复解析 source rule_set。

## 文件头建议

```text
magic: 4 bytes
version: u16
flags: u16
created_at: i64
rule_hash: [u8; 32]
section_count: u32
section_offsets: [u64; N]
```

校验策略：

- 读取 header。
- 校验 magic、version、endianness、section count、hash。
- 不做深度语义校验。

读取时只校验文件头必要参数。深度语义正确性由生成流程保证。
