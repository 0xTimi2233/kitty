# AGENTS.md

## Shell Rules

Always prefix shell commands with `rtk`.

Examples:

```bash
rtk git status
rtk cargo test
rtk npm run build
rtk pytest -q
```

Use `rtk proxy <cmd>` only when `rtk` does not support the command shape.

## Mini OMX Contract

This project uses the Mini OMX workflow in `docs/mini_omx/`.

When the user says one of these short commands, load the matching workflow automatically:

- `mini-intake`: use `docs/mini_omx/03-workflows/00-intake.md`
- `mini-plan`: use `docs/mini_omx/03-workflows/01-plan.md`
- `mini-spec`: use `docs/mini_omx/03-workflows/02-spec-and-test-plan.md`
- `mini-team`: use `docs/mini_omx/03-workflows/03-team.md`
- `mini-ralph`: use `docs/mini_omx/03-workflows/04-ralph.md`
- `mini-verify`: use `docs/mini_omx/03-workflows/05-verify.md`
- `mini-reset`: use `docs/mini_omx/03-workflows/06-milestone-reset.md`

## Source Of Truth

Do not keep project decisions only in chat context. Persist decisions and status in project files.

For Mini OMX work, use:

- `docs/mini_omx/readme.md`
- `docs/mini_omx/03-workflows/`
- `docs/mini_omx/04-templates/`
- project-specific `vision.md`, `roadmap.md`, `specs/*`, `status.md`

## Agent Routing

Prefer project custom roles before generic agents:

- `pm`
- `architect`
- `qa`
- `engineer`
- `verifier`
- `reviewer`

Use PM for requirements and roadmap, Architect for ADR and technical boundaries, QA for test-plan, Engineer for implementation, Verifier for evidence, and Reviewer for final risk review.

## Planning Gate

Do not implement non-trivial work unless the task has:

- approved status
- linked spec
- linked test-plan
- explicit write scope
- verification commands

If any item is missing, run `mini-plan` or `mini-spec` first.

## Team Rules

For parallel work:

- each worker gets exactly one task id
- each worker has disjoint write scope
- shared status files are updated by the main thread unless explicitly assigned
- every worker must report files touched, commands run, result, blockers, and remaining risks
- main thread performs integration verification

