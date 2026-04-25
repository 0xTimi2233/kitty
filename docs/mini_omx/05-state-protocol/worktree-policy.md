# Worktree Policy

默认不强制使用 git worktree。先用 write scope 隔离任务。

## 轻量模式

适用：

- 任务少于 3 个。
- 每个 task 修改不同目录或模块。
- 主线程能快速 review diff。

规则：

- 每个 worker 只能写自己的 write scope。
- 共享文件只能由主线程修改。
- `status.md` 由主线程最终更新。

## Worktree 模式

适用：

- 多个 worker 都会写代码。
- write scope 有潜在交叉。
- 任务可能跨多轮。

示例：

```bash
git worktree add ../project-worker-t001 -b worker/t001
git worktree add ../project-worker-t002 -b worker/t002
```

规则：

- 每个 worker 在自己的 worktree 中工作。
- worker 完成后提交本地 commit。
- 主线程负责 merge 或 cherry-pick。
- 冲突写入 `integration-report.md`。

## 禁止

- worker 直接 rebase 主分支。
- worker force push。
- worker 修改其他 worker 的 worktree。
- 未验证就合并。

