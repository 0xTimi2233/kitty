# 设计原则

## 1. 文档优先，不让上下文窗口承载项目状态

所有跨阶段信息都必须写入文件：

- `vision.md` 写目标和非目标。
- `gap-analysis.md` 写需求缺口和冲突。
- `roadmap.md` 写 milestone 和任务依赖。
- `adr/*.md` 写不可协商的架构决策。
- `specs/*.md` 写 feature contract。
- `specs/*-test-plan.md` 写验证矩阵。
- `status.md` 写任务状态、owner、验证证据和风险。

聊天上下文只用于当前 turn 的推理，不作为 source of truth。

## 2. 角色窄而硬

每个 role 只负责一个决策层：

- PM：目标、范围、优先级、milestone、status。
- Architect：架构边界、ADR、技术取舍、风险。
- QA：测试计划、验收矩阵、CI/readiness。
- Engineer：按 approved spec 实现。
- Verifier：证据化验证。
- Reviewer：最终 review，找 correctness / security / maintainability 风险。

不要让 Engineer 自己决定 scope，不要让 Verifier 顺手修代码。

## 3. Workflow 比 role 更重要

Role 定义“谁做事”，workflow 定义“事情按什么顺序发生”。Mini OMX 的 workflow 固定为：

```text
intake -> plan -> spec/test-plan -> ralph 或 team -> verify -> milestone reset
```

任何开发任务如果没有 approved spec 和 test-plan，应退回 planning。

## 4. Team 需要状态协议

多 agent 并行的核心不是“多开几个 agent”，而是可审查的状态：

- run id
- task id
- owner
- write scope
- status
- files touched
- commands run
- result
- blocker
- integration report

没有这些文件，就不要称为 team workflow。

## 5. 小任务用 ralph，大任务用 team

- `ralph`：一个 owner 执行一个 approved task 到完成。
- `team`：多个互不冲突的 approved tasks 并行执行。

默认优先 `ralph`。只有当 write scope 可以隔离时才用 `team`。

## 6. 里程碑边界重启

里程碑完成后，关闭旧 agent，重新从文档启动。不要让过期讨论、失败尝试和中间状态污染下一个 milestone。

