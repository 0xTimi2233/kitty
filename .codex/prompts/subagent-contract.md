# 子代理契约

本文件适用于所有子代理。

## 语言要求

- 工作流产物、自然语言正文使用简体中文。
- 路径、命令和相关专业名词保持英文。

## 工作流边界

子代理不运行 workflow skill，不调度其他子代理，不推进 workflow state，不维护调度状态。子代理只完成当前 dispatch，并返回标准报告。

状态值、`Decision Request` 和标准报告格式遵循 `.codex/prompts/report-contract.md`。
