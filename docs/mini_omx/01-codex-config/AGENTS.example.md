# AGENTS.md 示例

把本文件内容合并到项目根目录 `AGENTS.md`，或按项目需要改写。

## Mini OMX Contract

本项目使用 Mini OMX 工作流。主线程必须把项目状态写入文件，不得只保留在聊天上下文中。

## Source of Truth

开发前必须读取：

1. 项目根 `AGENTS.md`
2. `docs/mini_omx/README.md`
3. 当前 workflow 文件，例如 `docs/mini_omx/03-workflows/03-team.md`
4. 当前 task 对应的 `spec.md` 和 `test-plan.md`
5. 当前项目的 `status.md`

## Routing

优先使用项目自定义 roles：

- `pm`
- `architect`
- `qa`
- `engineer`
- `verifier`
- `reviewer`

不要让通用 default agent 替代这些角色，除非用户明确要求。

## Planning Gate

非平凡开发任务必须先有：

- approved roadmap item
- linked feature spec
- linked test-plan
- explicit write scope
- verification command list

缺少任一项时，退回 planning，不要实现。

## Team Rules

使用 subagents 并行时：

- 每个 worker 必须有独立 task id。
- 每个 worker 必须有 disjoint write scope。
- worker 不得修改未分配文件。
- worker 完成后必须报告 files touched、commands run、result、risk。
- 主线程负责 integration verification 和 status update。

## Verification

完成前必须运行与项目匹配的验证命令。若项目另有命令约束，以项目根 `AGENTS.md` 为准。

如果命令无法运行，必须在 status 中写明原因和残余风险。

