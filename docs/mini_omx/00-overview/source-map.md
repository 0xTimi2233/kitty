# 调研摘要

## OMX 可借鉴的部分

OMX 是 Codex CLI 的 workflow layer。它推荐的主路径是：

```text
$deep-interview -> $ralplan -> $ralph 或 $team
```

可借鉴点：

- canonical workflow：澄清、规划、执行、验证。
- team runtime 思想：任务分配、worker 状态、日志、resume、shutdown。
- `.omx/` 思想：计划、日志、memory、runtime state 落盘。
- hooks 思想：在 session start、prompt submit、tool use、stop 阶段做自动检查。
- doctor / hud 思想：用诊断和状态面板降低长任务失控风险。

不直接照搬的点：

- 不使用 OMX 默认角色。
- 不依赖 OMX npm package。
- 不把需求澄清放在长主线程上下文中。

## MaxLV 风格可借鉴的部分

MaxLV 的 mihomo-rust 实践核心是：

- 四角色分工：PM、Architect、Engineer、QA。
- 文件系统传递状态，而不是把协作塞进上下文窗口。
- ADR 决定架构，spec 填充细节，test-plan 验证 spec。
- milestone 驱动推进。
- milestone 边界重启 agent，避免 context rot。
- memory 只记录精简、可操作的 feedback。
- 测试是验证 agent 工作质量的唯一可靠手段。

Mini OMX 保留这些原则，并额外加入 Verifier 和 Reviewer。

## Codex 官方能力映射

Codex 官方 subagents 支持：

- 用 `/agent` 切换或查看 subagent thread。
- 明确要求时才会 spawn subagents。
- subagent 继承当前 sandbox policy 和 approval policy。
- 自定义 agent 放在 `.codex/agents/*.toml`，每个文件需要 `name`、`description`、`developer_instructions`。
- project `.codex/config.toml` 只在 trusted project 中加载。
- `AGENTS.md` 会按 global -> project root -> current directory 的顺序合并。
- `project_doc_max_bytes` 默认限制会影响 AGENTS 指令装载，长规则应拆成文档并按需读取。

Mini OMX 因此采用：

- `AGENTS.md` 放硬规则和入口指令。
- `.codex/agents/*.toml` 放窄角色。
- `docs/mini_omx/03-workflows/*.md` 放可复用流程。
- `docs/mini_omx/04-templates/*.md` 放可复制的项目文档模板。

## 主要来源

- OMX README: https://github.com/Yeachan-Heo/oh-my-codex/blob/main/README.md
- OMX skills: https://github.com/Yeachan-Heo/oh-my-codex/blob/main/docs/skills.html
- OMX agents: https://github.com/Yeachan-Heo/oh-my-codex/blob/main/docs/agents.html
- OMX native hooks: https://github.com/Yeachan-Heo/oh-my-codex/blob/main/docs/codex-native-hooks.md
- MaxLV Agent Team 实践: https://maxlv.net/blog/porting-mihomo-to-rust-with-claude/
- Codex subagents: https://developers.openai.com/codex/subagents
- Codex config basics: https://developers.openai.com/codex/config-basic
- Codex advanced config: https://developers.openai.com/codex/config-advanced
- Codex config reference: https://developers.openai.com/codex/config-reference
- Codex AGENTS.md: https://developers.openai.com/codex/guides/agents-md
- Codex hooks: https://developers.openai.com/codex/hooks

