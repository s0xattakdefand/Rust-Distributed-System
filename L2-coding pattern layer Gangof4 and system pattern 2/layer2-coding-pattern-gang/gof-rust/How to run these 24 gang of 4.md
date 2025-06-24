### Where to run Cargo commands

**Stand in the directory that holds the *workspace* `Cargo.toml`**
—in your tree that’s `gof-rust/` (the same folder that also contains `patterns/`, `scripts/`, `README.md`, etc.).

```bash
cd /path/to/gof-rust        # ← the folder with the top-level Cargo.toml
```

After you’re in that folder, Cargo understands all 24 member crates automatically because the root manifest’s `[workspace]` section lists `members = ["patterns/*"]`.

---

### Typical commands you’ll want

| Goal                             | Command                                 | What you’ll see                                                                   |
| -------------------------------- | --------------------------------------- | --------------------------------------------------------------------------------- |
| **Compile everything**           | `cargo check --workspace`               | Quick “does it build?” check.                                                     |
| **Run *all* unit-tests**         | `cargo test --workspace`                | Each pattern’s tests execute. Add `-- --nocapture` if you want `println!` output. |
| **Run tests for one pattern**    | `cargo test -p observer -- --nocapture` | Just the `observer` crate tests (replace `observer` with any crate name).         |
| **Generate docs for everything** | `cargo doc --workspace --open`          | Opens browser; each crate shows its API & examples.                               |
| **Lint all crates**              | `scripts/clippy.sh`                     | Runs `cargo clippy` for the whole workspace.                                      |
| **Format all code**              | `scripts/fmt.sh`                        | Runs `cargo fmt --all`.                                                           |

---

### Seeing “real output”

Right now each pattern crate is a **library** with unit-tests. The quickest way to watch something run is:

```bash
# run every test and keep println! output visible
cargo test --workspace -- --nocapture
```

Most of the sample tests included in the code (e.g. Observer, Mediator, Chain-of-Responsibility) print demo lines like `Logger saw value change to 42`, so you’ll get visible behaviour.

If you later add `examples/` to a crate (e.g. `patterns/observer/examples/demo.rs`), you can run it with:

```bash
cargo run -p observer --example demo
```

---

### One-liner sanity check

```bash
cd /path/to/gof-rust
cargo test --all --quiet && echo "✅ everything passes!"
```

If that prints the green ✅, your workspace is wired up correctly. From there you can start filling in more elaborate demos or integrating the crates into a bigger application.
