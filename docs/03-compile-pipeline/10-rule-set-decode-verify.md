# 10 Rule Set Decode Verify

## 职责

Rule Set Decode Verify 将加载到的 rule_set source 解码为规则 AST，或验证 KSR binary 的文件头并读取其中的编译数据。

## 输入

- `LoadedRuleSetSource`
- rule_set 配置。

## 输出

- `DecodedRuleSet`

## source format

source format 使用 JSON。解码后进入与主配置 rule 相同的 normalize 和 basic validate 路径。

## binary format

binary format 使用 KSR 文件。

KSR 校验只检查文件头必要参数：

- magic。
- format version。
- endian 标记。
- section count。
- payload length。
- header checksum 或 header hash。

KSR 文件由项目编译流程生成，深度语义正确性由生成流程保证，不在加载阶段重复执行完整校验。

## hash

持久文件完整性 hash 使用 BLAKE3。

## 错误处理

- source JSON 解码失败：当前编译失败。
- KSR 文件头非法：当前编译失败。
- KSR version 不兼容：当前编译失败。

## 测试要点

- source rule_set 能解码为 AST。
- KSR magic 错误会失败。
- KSR version 不兼容会失败。
- 合法 KSR 不执行重复深度语义校验。
