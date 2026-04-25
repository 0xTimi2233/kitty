# Workflow: Milestone Reset

用途：在 milestone 完成后重启 agent 上下文，避免 context rot。

## 触发条件

- 一个 milestone 全部 task 完成。
- team run 结束且 integration verification 通过。
- 主线程上下文明显过长。
- 需求或架构发生方向性变化。

## 步骤

1. PM 更新 `roadmap.md` 和 `status.md`。
2. QA 更新测试覆盖和 CI/readiness。
3. Architect 确认 ADR 是否需要补充。
4. Verifier 输出 milestone completion report。
5. 主线程生成 `milestone-summary.md`：
   - completed tasks
   - decisions made
   - tests run
   - remaining risks
   - next milestone entry criteria
6. 关闭或停止旧 agent。
7. 新会话只读取文档重新开始。

## 不要保存到 Memory 的内容

- 临时 debug 过程。
- 已经被代码表达的实现细节。
- 已经过期的任务状态。
- Git history。

## 可以保存到 Memory 的内容

- “不要做 X”的长期项目规则。
- 容易重复踩坑的外部库限制。
- 用户明确纠正过的偏好。

