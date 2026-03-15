---
name: review
description: Deep code review - Rust idioms, ownership, safety, tech debt. Launches 3 parallel reviewers + plan-reviewer gate. Use when: review, review this, code review, check this code, review my changes, is this good, what do you think, techdebt, tech debt, code smells.
---

# Code Review — Parallel Agents + Plan-Reviewer Gate

**Reviews current changes for correctness, idioms, tech debt, and Rust safety. Three parallel code-reviewer agents with focused scopes, aggregated findings, and a plan-reviewer critique before presenting fixes.**

## Workflow

### Phase 1: Diff

Get the full diff and diff stat against main:

```bash
git diff main...HEAD
git diff main...HEAD --stat
```

### Phase 2: Parallel Review

Launch **3 agents in parallel** (single message, 3 Agent tool calls). Each gets the full diff but a focused mandate.

**CRITICAL**: Launch all 3 agents in the **same message** so they run concurrently.

#### Agent 1: Correctness & Safety

```
prompt: |
  You are reviewing a Rust codebase (neural network library). Focus ONLY on correctness and safety.

  ## Your scope

  - Logic bugs, wrong matrix operations, incorrect dimensions
  - Ownership and borrowing issues — unnecessary clones, missing borrows
  - RefCell/Rc misuse — potential panics from double borrow
  - Unsafe code (if any) — soundness
  - Numeric overflow, precision loss in f64 operations
  - Off-by-one errors in matrix indexing
  - Dead code or unreachable paths
  - Missing error handling where operations can fail

  ## Output format

  Return findings as a markdown list, grouped by severity:

  ### Critical
  1) **Issue**: description
     **Location**: `file:line`
     **Fix**: solution

  ### Important
  A) **Issue**: description
     **Location**: `file:line`
     **Suggestion**: approach

  ### Minor
  * Nitpick or suggestion

  ### Positive
  - What's done well

  If no findings in a tier, omit that section. Cite file:line for every finding.
```

#### Agent 2: Idioms & Architecture

```
prompt: |
  You are reviewing a Rust codebase (neural network library). Focus ONLY on idioms and architecture.

  ## Your scope

  - Non-idiomatic Rust: manual loops where iterators fit, verbose match arms, unnecessary unwrap
  - Clone-heavy code — can it borrow instead?
  - Trait implementations — missing Display, From, Into where appropriate
  - Operator overloading consistency (Add, Sub, Mul on Matrix)
  - Module organization — god modules, misplaced code
  - DRY violations (same pattern 3+ times)
  - Over-engineering — unnecessary abstractions, single-use wrappers
  - Naming: domain-driven, descriptive, no single-letter vars except iterators

  ## Output format

  Same tier format as Agent 1. Cite file:line for every finding.
```

#### Agent 3: Completeness & Tests

```
prompt: |
  You are reviewing a Rust codebase (neural network library). Focus ONLY on completeness and tests.

  ## Your scope

  - New public behavior has corresponding tests in `tests/`
  - Tests cover edge cases (empty matrix, single element, mismatched dimensions)
  - Test naming is descriptive
  - Tests exercise real code, no mocking
  - Public API has doc comments with examples
  - CLAUDE.md reflects any structural changes
  - clippy warnings addressed

  ## Output format

  Same tier format as Agent 1. Cite file:line for every finding.
```

### Phase 3: Aggregate

After all 3 agents return, merge their findings:

1. **Merge** all findings into unified tiers (Critical / Important / Minor / Positive)
2. **Deduplicate** same file:line across agents
3. **Tag** each finding with source: `[safety]`, `[idioms]`, `[completeness]`
4. **Cap**: max 5 Critical, 7 Important, 3 Minor (drop lowest-impact excess)
5. **Verdict**: any Critical or Important → "Needs fixes"; only Minor/Positive → "Clean with suggestions"

### Phase 4: Plan + Critique

**If Critical or Important findings exist:**

1. Build a numbered fix plan:
   ```
   1. [severity] file:line — proposed fix
   2. [severity] file:line — proposed fix
   ```

2. Launch a **plan-reviewer agent** with the fix plan and diff stat to critique:
   - Are any fixes over-engineered?
   - Are there gaps the plan missed?
   - Are any fixes redundant or conflicting?
   - Would any fix break existing behavior?

3. **Incorporate critique**: drop over-engineered fixes, add missed gaps, adjust scope
4. Present the **critique-adjusted plan** to the user

**If only Minor findings**: skip plan-reviewer, present review directly.

### Phase 5: Present

```markdown
## Code Review — {branch name}

**Diff**: {files changed}, {insertions}+, {deletions}-

### Critical
1) [safety] **Issue**: description
   **Location**: `file:line`
   **Fix**: solution

### Important
A) [idioms] **Issue**: description
   **Location**: `file:line`
   **Suggestion**: approach

### Minor
* [completeness] Nitpick or suggestion

### Positive
- What's done well

### Verdict
[ ] Clean - ready for `/pr`
[x] Needs fixes - see plan below

---

## Fix Plan (critique-adjusted)

1. [Critical] `file:line` — fix description
2. [Important] `file:line` — fix description

*Plan reviewed by plan-reviewer. Dropped N over-engineered fixes, added M gaps.*
```

### Phase 6: Implement

After user approves the plan:

1. Implement fixes in plan order
2. Run verification:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```
3. Report results

## Pipeline

```
/dev <issue> -> /review -> /commit -> /pr
```
