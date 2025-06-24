# Gang-of-Four Patterns – Practical Rust Use-Cases 🦀

> A cheat-sheet you can scan in < 5 min when deciding *which* pattern solves
> *which* real problem.

---

## Creational Patterns

| Pattern | One-liner | Real-world Rust use-case | Why this pattern? |
|---------|-----------|--------------------------|-------------------|
| **Abstract Factory** | Create families of related objects without `new` everywhere. | Cross-platform GUI toolkit: a `GtkFactory`, `QtFactory`, `WebFactory` each builds buttons, windows and menus behind `dyn WidgetFactory`. | Swap entire widget families at runtime—no `cfg` spaghetti. |
| **Builder** | Incrementally assemble complex objects. | Build an `http::Request`: method → headers → body, **compile-time** guarantee the mandatory fields (`url`, `method`) are present. | Fluent API + avoids telescoping constructors. |
| **Factory Method** | Subclass decides what object to create. | Plugin system: each `dyn ParserPlugin` returns its own `Box<dyn AstNode>` when parsing a file. | Lets the *plugin* extend object hierarchy without touching core. |
| **Prototype** | Clone existing object to produce new ones. | Game engine clones a “bullet” prototype on every shot; texture buffers (`Arc`) are shared, mutable stats copied. | Faster than re-constructing; preserves exotic fields you might forget. |
| **Singleton** | Single shared instance for whole program. | `OnceLock<Config>` that loads YAML once and every crate calls `config()` to read values. | Thread-safe global without `unsafe`. |

---

## Structural Patterns

| Pattern | One-liner | Real-world Rust use-case | Why this pattern? |
|---------|-----------|--------------------------|-------------------|
| **Adapter** | Make incompatible interfaces play nice. | Wrap `OldLogger { write(&str) }` into `slog::Drain` so modern code logs through the old serial port API. | Preserve legacy code, expose modern trait. |
| **Bridge** | Decouple abstraction from implementation. | `RemoteControl<D: Device>` toggles `Tv`, `Radio`, or a fake `Fan` in tests. | Add new devices *or* new remotes independently. |
| **Composite** | Treat part-whole hierarchy uniformly. | `Directory`/`File` structs implement `trait Node { fn size() }` for a CLI *du-like* tool. | Recursion without RTTI; tree operations become one-liner iterators. |
| **Decorator** | Add behaviour without subclassing. | Chain `Bzip > Encrypt > TcpWrite` around a `File` implementing `trait Write`. | Runtime decide compression + encryption order; zero inherited bloat. |
| **Facade** | Unified interface over many subsystems. | `HomeTheater` facade turns on `Amp`, `Projector`, `Streamer` in correct sequence. | Keeps main app free from 6-step init boilerplate. |
| **Flyweight** | Share heavy immutable data. | Glyph cache: `GlyphFactory` interns `Arc<str>` for millions of syntax-highlighted characters. | Cuts memory by orders of magnitude. |
| **Proxy** | Stand-in that controls access. | `ImageProxy` lazily decodes a 4 K PNG only on first `.display()`, or `DbConnProxy` checks auth tokens before forwarding SQL. | Adds laziness, caching, security without touching real object. |

---

## Behavioral Patterns

| Pattern | One-liner | Real-world Rust use-case | Why this pattern? |
|---------|-----------|--------------------------|-------------------|
| **Chain of Responsibility** | Pass request along handlers until one deals with it. | HTTP middleware stack: `Auth → RateLimit → Logger → Handler`. | Clean pipeline, plug/unplug filters with zero glue. |
| **Command** | Encapsulate request as object. | GUI has `ToggleLamp`, `MoveShape`, `PasteText`; all go into an undo stack. | Uniform undo/redo & macro commands. |
| **Interpreter** | Evaluate sentences in a tiny language. | Firewall rule DSL: `"tcp && port == 443"`. AST nodes implement `.eval(&Packet)`. | New ops via enum variants; no big parser generator needed. |
| **Iterator** | Sequentially access collection without exposing internals. | Custom DB cursor streaming `Row` from a network socket implementing `Iterator<Item = Result<Row>>`. | Works with `for`, `map`, `collect`—ecosystem glue. |
| **Mediator** | Central object coordinates colleagues. | `EventBus` inside game: `Player`, `Enemy`, `UI` publish/subscribe without direct refs. | Slashes coupling; add new subsystem without editing each component. |
| **Memento** | Snapshot and restore object state. | Text editor saves `Editor::save()` before macro execute; restores on panic. | Rollback without exposing private fields. |
| **Null Object** | Safe no-op substitute. | `NullLogger` in tests where you don’t care about output but need `dyn Log`. | Kills `Option<Logger>` checks; avoids `if let Some(l) = …`. |
| **Observer** | Notify dependents on state change. | `Subject<Cell<i32>>` notifying WebSocket clients when config tweaks. | Reactive UI, live metrics, etc. |
| **State** | Behaviour changes with internal state. | `TrafficLight` FSM or `HttpParser` shifting through `StartLine`, `Headers`, `Body`. | Each state in its own `impl`; no monster `match` on enums. |
| **Strategy** | Swap algorithm at runtime. | Compression service chooses `Zip`, `Lz4`, `Zstd` at CLI flag; all impl `trait Compressor`. | Open/closed; benchmarks swap impls without touching call-sites. |
| **Template Method** | Skeleton algorithm with overridable steps. | `GameAI::turn()` calls hooks: `collect`, `build_structures`, `train_units`; `OrcsAI` overrides just `train_units`. | Enforces high-level flow while subclasses vary details. |
| **Visitor** | Separate operation from object structure. | `AreaVisitor` traverses shapes; extend with `SvgRenderVisitor` later. | Add new operations without touching big enum or every struct. |

---

### How to keep this file useful ✅

* **Link to code:** in each crate’s `README` add `See ../USE_CASES.md#pattern-name`.
* **Add numbers:** record memory saved, LOC removed, etc. when your team adopts a pattern.
* **Iterate:** as you refactor crates, update the real-world rows to reflect production stories.

Happy designing!
