2 · Pattern cheat-sheet

| Category   | Pattern crate             | Core Rust features to highlight                                       | Notes / Gotchas                          |
| ---------- | ------------------------- | --------------------------------------------------------------------- | ---------------------------------------- |
| Creational | `abstract_factory`        | trait objects, generic associated types (GATs)                        | compile-time factories vs dyn dispatch   |
|            | `builder`                 | **derive** – builder pattern via `typed-builder` or manual step enums | enforce invariants at compile time       |
|            | `factory_method`          | trait + `Box<dyn …>`                                                  | sample code below                        |
|            | `prototype`               | `Clone` + `Arc`                                                       | deep vs shallow clones                   |
|            | `singleton`               | `lazy_static` or `OnceLock`                                           | avoid `unsafe`; thread-safe global state |
| Structural | `adapter`                 | newtype pattern                                                       | wrap foreign API structs                 |
|            | `bridge`                  | decouple abstraction/implementation with traits + generics            | zero-cost at runtime                     |
|            | `composite`               | enums, recursive `Box`                                                | tree of components                       |
|            | `decorator`               | wrapper structs + `Deref`                                             | prefer composition over inheritance      |
|            | `facade`                  | public module re-exports                                              | hide messy subsystem crates              |
|            | `flyweight`               | interning with `Arc<str>` or `dashmap`                                | share immutable data                     |
|            | `proxy`                   | smart pointers (`Rc<RefCell<T>>`)                                     | virtual, protection, caching proxies     |
| Behavioral | `chain_of_responsibility` | iterator of handlers, `Option<&dyn Trait>`                            | each handler returns `Option`            |
|            | `command`                 | boxed closures, `Fn` trait objects                                    | undo/redo via command stack              |
|            | `interpreter`             | enum AST + visitor                                                    | parse small DSL                          |
|            | `iterator`                | custom `Iterator` impls                                               | idiomatic Rust—already built-in          |
|            | `mediator`                | channels (`tokio::sync::broadcast`)                                   | decouple component chat                  |
|            | `memento`                 | immutable snapshots (`serde_json`)                                    | store/restore state                      |
|            | `null_object`             | unit structs that no-op                                               | zero-cost default impl                   |
|            | `observer`                | `Vec<Weak<dyn Observer>>`                                             | sample code below                        |
|            | `state`                   | enum + `impl` blocks or dyn state objects                             | compile-time vs dynamic state            |
|            | `strategy`                | enum of strategies or trait objects                                   | swap behaviour at runtime                |
|            | `template_method`         | default trait methods + hooks                                         | override what varies                     |
|            | `visitor`                 | double dispatch via traits                                            | pattern matching + trait upcast          |
