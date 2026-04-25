# Codex 官方配置要点

本文件总结 Mini OMX 依赖的 Codex 官方配置能力。它不是完整配置手册，只记录和本工作流直接相关的部分。

## Config Precedence

Codex 配置优先级从高到低：

1. CLI flags 和 `--config`
2. selected profile
3. project `.codex/config.toml`
4. user `~/.codex/config.toml`
5. system config
6. built-in defaults

项目级 `.codex/config.toml` 只有在 trusted project 中才会加载。

## AGENTS.md Discovery

Codex 启动时会构建 instruction chain：

1. global `AGENTS.override.md` 或 `AGENTS.md`
2. project root 到当前目录路径上的 `AGENTS.override.md` / `AGENTS.md`
3. 越靠近当前目录的指令越晚出现，因此优先级更高

默认 `project_doc_max_bytes` 有大小限制。大型项目不要把所有规则塞进 AGENTS.md，应把规则放入 docs，然后在 AGENTS.md 中指定读取入口。

## Subagents

Codex subagents 默认可用，但只有用户明确要求时才会 spawn。

关键规则：

- subagent 会继承 parent session 的 sandbox 和 approval policy。
- inactive agent 的 approval request 也可能浮到主界面。
- custom agent 放在 `.codex/agents/*.toml`。
- 每个 custom agent 必须有 `name`、`description`、`developer_instructions`。
- `agents.max_threads` 默认是 6。
- `agents.max_depth` 默认是 1，建议保持 1，避免 recursive fan-out。

## Sandbox And Approval

常用组合：

- planning / review：`sandbox_mode = "read-only"`
- normal implementation：`sandbox_mode = "workspace-write"` + `approval_policy = "on-request"`
- full access：`danger-full-access` + `never`，只在外部环境已经隔离时使用

Mini OMX 推荐：

```toml
approval_policy = "on-request"
sandbox_mode = "workspace-write"

[agents]
max_threads = 6
max_depth = 1
```

## Project Agents

推荐把通用角色放入：

```text
.codex/agents/
  pm.toml
  architect.toml
  qa.toml
  engineer.toml
  verifier.toml
  reviewer.toml
```

不要覆盖内置 `default`、`worker`、`explorer`，除非你明确想替换它们。

## Hooks

Codex hooks 可以放在：

- `~/.codex/hooks.json`
- `~/.codex/config.toml`
- `<repo>/.codex/hooks.json`
- `<repo>/.codex/config.toml`

常用事件：

- `SessionStart`
- `UserPromptSubmit`
- `PreToolUse`
- `PostToolUse`
- `PermissionRequest`
- `Stop`

Mini OMX 当前不要求 hooks。后续如果要自动化，可以用 hooks 做：

- 阻止没有 approved spec 的实现请求。
- 在 `Stop` 阶段提醒更新 status。
- 在 `PreToolUse` 阶段阻止 destructive shell command。
- 在 `PostToolUse` 阶段提示记录 verification evidence。

## Slash Commands

对 Mini OMX 最有用的 Codex slash commands：

- `/agent`：查看或切换 subagent thread。
- `/plan`：先进入 plan mode。
- `/review`：对当前 working tree 做审查。
- `/diff`：查看 diff。
- `/status`：查看当前 model、approval、sandbox、context。
- `/compact`：长会话时压缩上下文。
- `/new`：在同一个 CLI 中开启干净上下文。

## 使用建议

- 需求和 planning 阶段优先用短会话，产物写文件。
- 开发阶段从 `status.md` 和 spec/test-plan 开新会话。
- 里程碑结束后用 `/new` 或重启 Codex，不带旧上下文继续。
- 长 team run 必须写 run state 文件，不能只依赖 `/agent` thread。

