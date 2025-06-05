# 🫀 Pattern 2: Heartbeat / Health Check

---

## 👶 Explain to Me Like I’m 5

> Imagine you and your friends are playing hide and seek.

Every few seconds, your friends yell:

> “I’m still here!”

If you don’t hear them for a while, you think:

> “Uh-oh! They must have gone home or got tired.”

💡 That yelling is called a **heartbeat** — a way for your computer friends (servers) to say:

> “I'm alive and still playing!”

If a computer **stops sending heartbeats**, it means:

- It might be broken 💥
- It might be asleep 😴
- Or it lost internet 🌐

So we take action: ❗ Notify, Restart, or Replace.

---

## 🧠 What Is Heartbeat / Health Check?

A **heartbeat** is a small, regular signal sent from a node or service to say:

> ✅ “I’m alive!”

A **health check** is a way to **ask a service**:

> “Are you still working okay?”

---

## 🔧 How It Works

| Component      | Role                                                                 |
|----------------|----------------------------------------------------------------------|
| **Node (client)**     | Sends heartbeats to a central server (`POST /heartbeat`)            |
| **Manager (server)**  | Tracks latest heartbeat times and detects if any node is “silent”   |
| **Dashboard**         | (Optional) Displays active nodes or failed ones                    |

---

## 🧰 Real-World Use Cases

| System               | Purpose                                                               |
|----------------------|------------------------------------------------------------------------|
| **Kubernetes**        | Uses `livenessProbe` and `readinessProbe` to restart dead containers   |
| **Redis Sentinel**    | Monitors Redis master via heartbeats and promotes a new master if down |
| **Consul**            | Health checks for microservices (HTTP, TCP, scripts)                   |
| **Netflix Eureka**    | Clients send heartbeats to stay in the registry                        |
| **Load Balancers**    | Drop servers from pool if health check fails                           |
| **IoT Devices**       | Ensure devices are still connected and responsive                      |
| **Multi-player Games**| Track which players are still connected                                |

---

## 🧩 When to Use It

Use **Heartbeat / Health Check** when you want to:

- Detect if a **service or device crashes** or disappears 🔍
- Build a **monitoring system** to track live nodes 📡
- Auto-recover or restart parts of a **distributed system** 🔁
- Show **real-time dashboards** of connected services 👁️
- Make sure **critical systems** like databases or APIs are always working ✅

---

## 🚨 Example: What Happens Without It?

> You run 5 services across 3 continents...  
> One crashes silently at 3 AM...  
> Nobody notices until a customer complains.

With **heartbeats**:

- You detect the failure in **seconds**
- Automatically **replace** or **restart**
- Send alerts to the team

---

## ✅ Summary

| Term         | Meaning                                 |
|--------------|------------------------------------------|
| Heartbeat    | A message saying "I'm alive!"            |
| Health Check | Asking "Are you okay?"                   |
| Use Case     | Detect failures, restart, monitor        |
| Kids Example | “Yelling every 5 seconds so friends know you didn’t go home” |

---

## 💡 Next Steps (Ideas to Build)

- Add `/healthz` endpoint to report app status
- Track uptime duration per node
- Show live UI with blinking nodes 🟢 🔴
- Push alerts if any node misses 3+ heartbeats
- Use SQLite or Redis to store heartbeat logs

---

🦀 Built in **Rust** using **Axum + Tokio**  
💻 Ready for distributed systems, clusters, and microservices!

