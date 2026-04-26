# 术语

- ACL：Anti-Corruption Layer，配置输入防腐层。
- SchemaConfig：ACL 反序列化后的配置输入模型。
- Domain Model：控制面和数据面共享的核心领域模型。
- Runtime Model：compile-pipeline 发布给数据面的只读驻留模型。
- Compile Pipeline：从配置解码到 runtime atomic publish 的控制面流程。
- Match Pipeline：数据面按 runtime model 完成流量匹配和 action dispatch 的流程。
- KSR：Kitty 自定义 rule_set binary 格式。
- CacheKeepSet：发布成功后需要保留的缓存对象集合。
