# Runtime Model

Runtime Model 是 compile-pipeline 发布给数据面的只读驻留模型。

要求：

- 发布后不可变。
- action target、detour、final target 在编译期固化。
- matcher index、rule order、rule metadata、string intern table 在编译期完成。
- 数据面通过 generation handle 原子切换。
- 旧 generation 在没有数据面引用后释放。
