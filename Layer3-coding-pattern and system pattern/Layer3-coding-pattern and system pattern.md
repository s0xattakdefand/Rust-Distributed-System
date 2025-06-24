### Layer 3 – *Macro* patterns

*Layer 3 zooms out one more step: instead of single crates (Layer 1) or single services (Layer 2), we’re arranging **whole subsystems and domains**.  Two viewpoints again—“coding” (internal software architecture) and “system” (inter-service & infrastructure architecture).*

---

## 3-A Coding-architecture patterns (inside an **application** but above class-level)

| Pattern                                 | Core idea                                                                                           | Typical Rust scope                                                                       |
| --------------------------------------- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| **Hexagonal / Ports-and-Adapters**      | Domain logic sits in the centre; all IO is an adapter implementing inbound/outbound ports (traits). | Put domain in `core/`, adapters in `infra/`, wire in `main.rs`.                          |
| **Clean / Onion**                       | Concentric rings: entities → use-cases → interfaces; dependencies always point inwards.             | Crate or module boundaries match rings, enforced with `pub(crate)` & trait boundaries.   |
| **Micro-kernel / Plugin**               | Lightweight core loads feature plugins at runtime.                                                  | Dynamic-dispatch (`dyn Plugin`) + `libloading`/`wasmtime` for hot-swap.                  |
| **CQRS + Event Sourcing (app flavour)** | Separate command model from query model; persist domain events.                                     | One crate owns `events/`, another owns `read_model/`; builder replays events into views. |
| **MVC / MVVM**                          | Split UI, presentation, domain; still relevant for Tauri / Yew apps.                                | Component structs in `ui/`, commands send to domain service over channels.               |
| **Strangler-Fig Refactor (code)**       | Incrementally wrap legacy module behind an interface and replace slice-by-slice.                    | New Rust façade crate delegates to old C ABI; slices replaced with Rust impls over time. |

---

## 3-B System-architecture patterns (across **multiple services**)

| Cluster-level pattern                          | What it solves                                                             | 2025 Rust/cloud example                                                    |
| ---------------------------------------------- | -------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| **Micro-services Mesh**                        | Uniform cross-cutting (mTLS, retries, tracing) via sidecars/Linkerd/Istio. | Each Rust svc ships without HTTP client retries—mesh handles it.           |
| **Event-Driven Architecture (EDA)**            | Services exchange events, stay loosely coupled.                            | Axum producers → NATS JetStream → Tokio consumer workers.                  |
| **Data Mesh**                                  | Federated, domain-owned analytical data; discoverable via catalog.         | Each domain emits Protobuf schemas; Rust Spark jobs read via Delta-Lake.   |
| **Saga-Orchestrated Workflow**                 | Long business transaction spanning 5-6 services with rollback.             | Our Layer 2 `saga` crate becomes part of a dedicated Orchestrator service. |
| **API Composition / Backend-Aggregator**       | Aggregate N micro-calls into one boundary call.                            | Actix-BFF (Layer 2) stitched into Edge GraphQL gateway.                    |
| **Strangler-Fig (system)**                     | Route only some endpoints to the new Rust service, rest to monolith.       | Envoy/Traefik header-based routing + new checkout-rs.                      |
| **Service Mesh Canary / Progressive Delivery** | Mesh or Rollout controller shifts traffic by metric.                       | Link up Layer 2 *canary* crate with Argo Rollouts CRD.                     |
| **Data Pipeline / Lakehouse**                  | Immutable ingestion ➜ bronze/silver/gold tables, governed schema.          | Rust + Apache Iceberg ingestion, Delta-Kernel query.                       |

---

## How Layer 3 patterns work **with** Layer 2 & Layer 1

* **Inside** each Layer 3 coding ring (Hexagonal, Clean, etc.) you still use
  Layer 2 class-level GoF patterns (Strategy for algorithms, Observer for events).
  Those GoF patterns keep the inner ring maintainable.

* **Across** Layer 3 system slices (EDA, Mesh) you still employ Layer 2 system
  utilities—circuit-breaker, auto-scaler, canary—at every service boundary.

> **Analogy:**
> *Layer 1* = bricks, *Layer 2* = walls, *Layer 3* = whole buildings in the city.
> Good cities need both solid walls **and** thoughtful urban planning; so do resilient software platforms.
