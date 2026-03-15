---
description: Git operations — committing, branching, staging
globs: ["**/*"]
alwaysApply: false
---

# Git Conventions

## Commits

Format: `<type>: <short description>`

Types: `feat:`, `fix:`, `refactor:`, `test:`, `chore:`, `docs:`

Rules:
- Present tense ("add" not "added"), lowercase after prefix
- Never mention AI/Claude in commits
- Never add Co-Authored-By trailers
- No emojis in commit messages

## Staging

- `git add <specific files>` — never `git add .` or `git add -A`
- Review staged diff before committing: `git diff --staged`

## Branches

- `feat/<name>` — new features
- `fix/<name>` — bug fixes
- `refactor/<name>` — refactoring
- `chore/<name>` — maintenance

## Pre-commit

```bash
cargo test      # all tests pass
cargo clippy    # no lint warnings
cargo fmt --check  # formatting check
```

## Examples

```bash
git add src/matrix.rs tests/matrix_test.rs
git commit -m "feat: add matrix transpose operation"

git add src/layer.rs
git commit -m "fix: handle zero-sized layer initialization"

git add src/neural_network.rs src/layer.rs
git commit -m "refactor: extract weight update into layer method"
```
