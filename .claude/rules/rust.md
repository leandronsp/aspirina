---
description: Rust patterns, idioms, and anti-patterns
globs: ["src/**/*.rs", "tests/**/*.rs"]
alwaysApply: false
---

# Rust Patterns

## Ownership & Borrowing

- Prefer borrowing over cloning. Clone only when ownership transfer is necessary.
- Use `&self` by default, `&mut self` when mutation is needed, `self` only for consuming methods.
- Prefer `&str` over `&String`, `&[T]` over `&Vec<T>` in function parameters.

## Control Flow

- **`match`** for exhaustive pattern matching. No catch-all `_ =>` unless genuinely unreachable.
- **`if let`** for single-variant matching.
- **Early returns** over nested conditionals.
- No `unwrap()` in library code. Use `?` operator for propagation.

## Types & Traits

- `#[derive(Debug)]` on all structs.
- Implement `Display` for user-facing types.
- Newtype pattern for domain types over primitives.
- Prefer composition over trait inheritance.

## Naming

- Descriptive variable names, no single-letter vars except iterators.
- Domain-driven naming: types over primitives.
- `snake_case` for functions/variables, `PascalCase` for types/traits.
- Builder pattern: `with_x()` methods returning `Self`.

## Error Handling

- Custom error types per module when needed.
- `Result<T, E>` over panics. Reserve `panic!` for truly unrecoverable states.
- `?` operator for error propagation, not manual `match` on `Result`.

## Architecture

- Modules <200 lines. Extract submodules when growing.
- `pub` only what needs to be public. Default to private.
- Thin `main.rs`, domain logic in `lib.rs` and dedicated modules.

## Anti-Patterns

- `unwrap()` / `expect()` in library code
- `clone()` to avoid borrow checker instead of restructuring
- God modules >200 lines
- `pub` on everything
- Defensive guards on internal code
- Commenting obvious code
