🛰️ How Many Types of Service Discovery?
Here’s the complete list of Service Discovery types, categorized by architecture:

| # | Type                      | Description                                                         | Example Technologies                  |
| - | ------------------------- | ------------------------------------------------------------------- | ------------------------------------- |
| 1 | **Client-Side Discovery** | The client asks a registry and picks a service instance itself      | Netflix Eureka, Consul + custom Rust  |
| 2 | **Server-Side Discovery** | The load balancer talks to the registry and forwards requests       | Kubernetes Service, AWS ALB, Linkerd  |
| 3 | **DNS-Based Discovery**   | Uses DNS records to resolve service IPs or hostnames                | Kubernetes DNS, Consul DNS            |
| 4 | **Sidecar Discovery**     | A sidecar (proxy container) handles all discovery logic for the app | Istio Envoy, Linkerd-proxy            |
| 5 | **Hybrid Discovery**      | Mixes client-side + server-side + DNS as fallback                   | Istio + DNS fallback                  |
| 6 | **Broadcast/Multicast**   | Services announce themselves on the network (only in small LANs)    | mDNS, Avahi, etc. (not used in cloud) |
| 7 | **Static Discovery**      | Pre-configured list of IPs or hosts (manual, not scalable)          | Config files or `.env` hardcoded URLs |

### project structure of the level5 service-discovery

rust-discovery/
├── service_registry/
│   └── register.rs           # Registers the service to Consul
├── service_discovery/
│   └── discover.rs           # Looks up another service from Consul
├── services/
│   ├── inventory/
│   │   └── main.rs           # A sample Inventory microservice
│   └── order/
│       └── main.rs           # A sample Order microservice (client)
├── shared/
│   └── http_utils.rs         # Shared HTTP request helpers
├── Cargo.toml
└── README.md
