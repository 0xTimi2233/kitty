# KSR Binary Format

KSR 是 Kitty 自定义 rule_set binary 后缀。

文件头至少包含：

- magic。
- version。
- endian。
- rule count。
- section offsets。
- content hash。

读取 KSR 时只校验文件头必要参数。深层语义由生成流程保证。
