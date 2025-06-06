There are **6 major types** of the **Replica Pattern** in distributed systems. Each has unique use cases based on **consistency, availability, and performance** requirements.

---

### 🧩 Replica Pattern Types & Use Cases

| # | 🧠 Type                      | 📚 Description                                                        | 🌍 Real-World Use Case                                     |
| - | ---------------------------- | --------------------------------------------------------------------- | ---------------------------------------------------------- |
| 1 | **Master-Slave Replication** | One node (master) handles writes, others (slaves) replicate reads     | ✅ Databases like MySQL (read scaling, write durability)    |
| 2 | **Multi-Master Replication** | Multiple nodes can perform writes, sync changes across replicas       | 🔄 CRDTs, real-time collaboration tools like Google Docs   |
| 3 | **Leader-Follower**          | Leader node processes all requests and replicates to followers        | 🗳️ Kafka brokers, Raft consensus-based systems            |
| 4 | **Read-Replica**             | One primary node, multiple read-only replicas for query offloading    | 📈 API backends, analytics systems (scaling reads)         |
| 5 | **Chain Replication**        | Requests move in a chain from head to tail for consistency guarantees | 🧵 Cassandra-like write path, durable writes with ordering |
| 6 | **Quorum-Based Replication** | Data must be written/read from a majority (quorum) of replicas        | ⛓️ Blockchain consensus (e.g., Tendermint, Raft, Paxos)    |

---

### 🧒 Explain Like I'm 5

> Imagine you have a favorite coloring book, but all your friends want to read it too.

Here’s how each type works:

1. **Master-Slave:**
   Daddy reads the story (master), and mommy and grandma copy it down (slaves). Only daddy writes new parts.

2. **Multi-Master:**
   Everyone can write in their own book. Later, they show each other and fix differences.

3. **Leader-Follower:**
   Only one friend leads the story. Others follow and copy everything the leader does.

4. **Read-Replica:**
   You draw in your book. Your friends only get to look, not draw.

5. **Chain Replication:**
   One person tells a story to the next, who tells the next, like whispering in a circle. The last one saves it forever.

6. **Quorum-Based:**
   Everyone votes on what to write next. If most agree, then it goes into the book.

---

Would you like me to implement any of these types next in Rust with a complete project structure?
