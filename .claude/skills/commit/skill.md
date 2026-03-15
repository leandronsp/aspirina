---
name: commit
description: Create a git commit following project conventions. Use when: commit, commit this, make a commit, commit changes, git commit, save changes, commit my work, stage and commit, detailed commit.
---

# Git Commit

## Modes

### Quick (default)

Single-line commit message.

```
<type>: <short description>
```

### Detailed (`/commit detailed` or `/commit -d`)

Multi-paragraph commit for significant changes. Review the conversation and staged diff, then write:

```
<type>: <short summary>

<paragraph explaining what changed and why>

<paragraph on technical approach, trade-offs, or notable decisions>
```

Use detailed mode for: milestone features, non-obvious fixes, architectural changes, anything where "why" matters more than "what".

## Format

Types: `feat:`, `fix:`, `refactor:`, `test:`, `chore:`, `docs:`

## Rules

1. **Concise** — short message, present tense ("add" not "added")
2. **Lowercase** after prefix
3. **No AI mentions** — never reference Claude, AI, or assistants
4. **No Co-Authored-By** — never add Co-Authored-By trailers
5. **No emojis** in commit messages
6. **Specific files** — `git add <files>`, never `git add .`

## Pre-commit Checklist

```bash
cargo test
cargo clippy
cargo fmt --check
git diff --staged
```

## Examples

### Quick

```bash
git commit -m "feat: add batch matrix multiplication"
git commit -m "fix: handle empty matrix in transpose"
git commit -m "refactor: extract activation into trait"
```

### Detailed

```bash
git commit -m "feat: add learning rate decay to training

Training now supports configurable learning rate decay, reducing the
rate by a factor each epoch. This prevents overshooting minima in
later training stages when weights are already close to optimal.

Decay is optional and defaults to 1.0 (no decay). Each gate training
scenario uses hand-tuned decay values for faster convergence."
```
