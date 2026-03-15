---
description: Rust testing and TDD conventions
globs: ["tests/**/*", "src/**/*"]
alwaysApply: false
---

# Testing Conventions

## Running Tests

```bash
cargo test                        # all tests
cargo test matrix_test            # single test file
cargo test matrix_test::test_add  # single test
cargo test -- --nocapture         # with stdout
cargo clippy                      # lint (type-level checks)
```

## TDD Cycle

1. **RED** — Write the test asserting correct behavior, run it, confirm it fails
2. **GREEN** — Write minimum code to make it pass
3. **REFACTOR** — Clean up while staying green
4. Repeat

### Tests drive code, never the reverse

- The test defines what correct behavior is. Never change a test to match a wrong implementation.
- If the implementation returns wrong results but the test expects correct ones, fix the implementation.
- Every branch, every edge case must have a test that fails without the corresponding code.

## Conventions

- Write tests BEFORE or alongside implementation, never after
- Test public API only. No testing private functions.
- One assertion focus per test (multiple asserts OK if one logical thing)
- `mod tests` blocks in test files, grouped by function/feature
- Shared setup via helper functions in test files

## Edge Cases to Test

- Empty matrices (0x0)
- Mismatched dimensions (should error or handle gracefully)
- Single element matrices
- Large values / overflow behavior
- Identity operations (multiply by identity matrix, add zero matrix)
