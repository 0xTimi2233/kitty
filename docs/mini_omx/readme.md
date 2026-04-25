# Mini OMX Workflow

Mini OMX 是一套不依赖 `oh-my-codex` 安装的项目级 agent workflow。它借鉴三类实践：

- OMX 的核心 workflow：先澄清需求，再审计划，再执行，再验证；执行阶段保留 team 状态跟踪。
- MaxLV 风格：PM / Architect / QA / Engineer / Verifier 分工，所有跨 agent 状态通过文件系统交接。
- OpenAI Codex 官方能力：`AGENTS.md`、project `.codex/config.toml`、`.codex/agents/*.toml`、`/agent`、subagents、sandbox 和 approval 配置。

它的目标不是复制 OMX runtime，而是提供一套可控、可审查、可恢复的文档驱动流程。

## 目录

```text
docs/mini_omx/
  00-overview/          设计原则和调研摘要
  01-codex-config/      Codex 配置示例，不会自动生效
  02-roles/             自定义 subagent TOML 示例
  03-workflows/         可直接交给主线程执行的 workflow prompt
  04-templates/         项目文档模板
  05-state-protocol/    team 状态协议和 worktree 策略
  06-example-vocab-app/ 背单词 Web App 示例项目文档
```

## 推荐使用顺序

1. 需求澄清：

```text
使用 docs/mini_omx/03-workflows/00-intake.md。
假设我要做一个背单词 Web App。
由 pm 角色产出 vision、requirements、non-goals 和 acceptance criteria。
只写 docs，不写代码。
```

2. 规划：

```text
使用 docs/mini_omx/03-workflows/01-plan.md。
由 architect、pm、qa 顺序协作，生成 roadmap、ADR、feature specs 和 test plans。
只规划，不实现。
```

3. 单任务执行：

```text
使用 docs/mini_omx/03-workflows/04-ralph.md。
执行 status.md 中第一个 approved task。
实现前读取 spec 和 test-plan，完成后更新 status，并记录验证命令。
```

4. 多任务并行：

```text
使用 docs/mini_omx/03-workflows/03-team.md。
从 status.md 中选择最多 3 个互不冲突的 approved tasks。
创建 team run 状态目录，调度 engineer subagents 并跟踪 worker 状态。
```

5. 验证：

```text
使用 docs/mini_omx/03-workflows/05-verify.md。
由 verifier 和 reviewer 只读验证当前变更是否满足 spec、test-plan 和 acceptance criteria。
```

## 核心规则

- 主线程只做 orchestration，不长期承载需求讨论。
- 文档是 source of truth，不把中间结论藏在聊天上下文里。
- PM 管 roadmap 和需求优先级。
- Architect 管 ADR 和不可协商的架构决策。
- QA 管 test-plan 和 CI/test coverage。
- Engineer 只实现 approved spec。
- Verifier 只验证，不继续开发。
- 每个 team run 都必须有状态文件，不能只靠口头描述。
- 里程碑结束后重启 agent，从文件系统重新加载上下文。

## 和 OMX 的关系

Mini OMX 借鉴 OMX 的 team 思路，但不依赖 OMX：

| 能力 | OMX | Mini OMX |
|---|---|---|
| `$deep-interview` | 内置 skill | `00-intake.md` |
| `$ralplan` | 内置 planner / architect / critic | `01-plan.md` |
| `$ralph` | 持久执行 loop | `04-ralph.md` |
| `$team` | tmux / worktree / status runtime | `03-team.md` + 文件状态协议 |
| `.omx/state` | OMX runtime 状态 | `docs/mini_omx` 模板 + 项目内 run 状态 |

Mini OMX 的 `$team` 是主线程调度协议，不是后台 runtime。需要真正的 tmux、resume、HUD、自动合并时，再考虑 OMX 或自研 CLI。

