# Advent of Code

## `xaoc` Guide

### Setup
*   **Install/Update**: `xaoc` is managed via `flake.nix`. Ensure `xaoc` is in your dev shell.
*   **Auth**: Add your session token: `xaoc auth add <session-token>`
    *   Get the session cookie from your browser dev tools on adventofcode.com.
*   **Config Dir**: `~/.config/xaoc` (stores tokens, inputs, puzzle text).

### Daily Workflow
1.  **Start a Day**: `xaoc prepare <day>`
    *   Fetches puzzle text and input.
    *   Creates `src/bin/<year>_<day>.rs` from template (if not exists).
    *   Example: `xaoc prepare 1`

2.  **Run Code**:
    *   `cargo run --release --bin <year>_<day>`
    *   Add `-- --p1` or `-- --p2` to run specific parts.
    *   Add `-- --sample <n>` to run with a specific sample input extracted from the puzzle text.
    *   Add `-- --dev` to run with sample inputs (extracted from puzzle text).

3.  **Submit**: `xaoc submit <day> <part> <answer>`
    *   Example: `xaoc submit 1 1 12345`
    *   Automatically checks against known bad answers locally before submitting.
    *   Updates local answer cache on success.

### Xaoc Macro

The `xaoc!` macro customizes how your solution runs, particularly for sample inputs.

**Keys:**
*   `sample_idx`: (usize) 0-indexed position of sample input from puzzle description.
*   `sample_idx2`: (Option<usize>) 0-indexed position for part 2's sample input.
*   `sample`: (&'static str) Custom sample input string for part 1.
*   `sample2`: (&'static str) Custom sample input string for part 2.
*   `no_sample`: (bool) Disable all sample input processing.

**Example:**
```rust
xaoc!(
    sample_idx = 0,
    sample = "custom sample input for part 1",
    sample2 = "custom sample input for part 2"
);
```

### Other Commands
*   **Sync Answers**: `xaoc sync-answers`
    *   Fetches your solved answers from the website and populates local cache.
*   **List Tokens**: `xaoc auth list`

### Project Structure
*   `aocYYYY/`: Year-specific crates.
*   `xaoc/`: Shared library and CLI tool.
*   `xaoc/fixtures/template.rs`: Template for new day binaries.
