# Aspirina

Neural network library in Rust. The core is matrix operations and layers with feedforward/backpropagation. Everything else are examples that exercise the library.

The `src/computer/` module is a sample project: a 4-bit neural CPU where all arithmetic/logic is performed by trained neural networks. Extracted to [Synapse](https://github.com/leandronsp/synapse).

## Project Structure

```
├── src/
│   ├── lib.rs                # Re-exports core modules
│   ├── main.rs               # Interactive menu
│   ├── matrix.rs             # Matrix type with operator overloading, transpose, element-wise ops
│   ├── calc.rs               # Sigmoid, tanh with derivatives
│   ├── layer.rs              # Single network layer: weights + cached forward result
│   ├── neural_network.rs     # Rc<RefCell<Layer>> shared layers, forward/backward, train, predict
│   ├── training/             # Logic gate training scenarios (XOR, AND, OR, etc.)
│   └── computer/             # 4-bit neural CPU (gates, ALU, memory, CPU, assembler)
├── tests/
│   ├── matrix_test.rs
│   ├── calc_test.rs
│   ├── layer_test.rs
│   └── neural_network_test.rs
├── .claude/
│   ├── agents/
│   │   ├── scout.md          # Read-only codebase explorer
│   │   ├── code-reviewer.md  # Staff Engineer reviewer (Rust)
│   │   └── plan-reviewer.md  # Implementation plan stress-tester
│   ├── rules/
│   │   ├── git.md            # Git conventions (commits, branches, staging)
│   │   ├── testing.md        # TDD conventions
│   │   └── rust.md           # Rust patterns and anti-patterns
│   └── skills/
│       ├── commit/skill.md   # Git commit
│       ├── dev/skill.md      # TDD implementer
│       ├── review/skill.md   # Code review
│       ├── po/skill.md       # Product Owner (GitHub issues)
│       └── pr/skill.md       # Pull request creator
└── CLAUDE.md
```

## Build & Run

```bash
cargo build              # Build
cargo run                # Interactive menu (gate training + computer tests)
cargo run --release      # Optimized build
cargo test               # All tests
cargo test matrix_test   # Single test file
cargo test -- --nocapture # Tests with stdout
cargo fmt                # Format
cargo clippy             # Lint
```

## Code Standards

- Self-documenting function names; comment non-obvious logic (WHY, not WHAT)
- `cargo test` must pass before committing
- `cargo clippy` must pass before committing
- `cargo fmt --check` must pass before committing
- No external deps unless strictly necessary

## TDD

- Write tests BEFORE or alongside implementation, never after
- Run `cargo test` after every meaningful change, not just at the end
- Test public API only. Do not test private functions directly.
- One assertion focus per test (multiple asserts OK if testing one logical thing)

## Git

- Conventional commits: `feat:`, `fix:`, `refactor:`, `test:`, `chore:`
- Never mention AI/Claude in commits, no Co-Authored-By
- Stage specific files, never `git add .`

## Key Design Decisions

### Core
- **Weight matrix shape**: rows = neurons in current layer, columns = inputs. First layer is `hidden_size x input_size`.

### Computer Example (Synapse)
- **4-bit constraint**: All values masked with `& 0x0F`. Only 16 memory locations (0x0-0xF).
- **Memory layout**: Programs in low memory (0x0-0x8), temporaries in high memory (0xD-0xF). Overlapping causes silent corruption.
- **Two-byte instructions**: Each CPU instruction is opcode + operand. PC increments must account for this.
- **Operator precedence in interpreter**: Uses `rfind()` for left-to-right evaluation so `a - b + c` parses as `(a - b) + c`.
