# 🌀 Pattern 4: Consistent Hashing

---

## 👶 Explain to Me Like I’m 5

> Imagine you have 4 friends: 🧒🧒🧒🧒  
> You have 100 toys 🧸🪀🧩 and you want to give each toy to one friend.

You say:
> “Let’s use a magic wheel to decide which friend gets which toy!”

So you spin a wheel (hash function), and it points to Friend #2.

If one of your friends goes home 🏠, you don’t want to spin the wheel again for ALL toys — that would be a lot of work 😩.

Instead:
> “Just give the toys from the missing friend to the next friend around the circle.”

That’s **consistent hashing**! 🎯

It helps you:
- Find where to put things
- Quickly adjust when someone leaves or joins
- Without moving everything again

---

## 🧠 What Is Consistent Hashing?

**Consistent hashing** is a smart way to assign keys (like files, users, tasks) to nodes (like servers or caches) with **minimal data movement** when nodes are added or removed.

Instead of splitting keys by range (which causes lots of reshuffling), it arranges nodes in a circle and hashes both nodes and keys onto that ring.

---

## 🧰 Real-World Use Cases

| System              | What It's Used For                                    |
|---------------------|--------------------------------------------------------|
| **Amazon DynamoDB** | Store and retrieve partitioned data                   |
| **Cassandra DB**    | Distribute rows across shards                         |
| **Memcached**       | Route cache keys to the right cache node             |
| **Akka Cluster**    | Shard actors across nodes                            |
| **CDNs / DNS**      | Pick the nearest or responsible edge server          |
| **Kafka Partitions**| Assign messages to partitions with key consistency   |

---

## 🧩 Why Use Consistent Hashing?

| Problem                                | Consistent Hashing Solves It By...                  |
|----------------------------------------|-----------------------------------------------------|
| A server goes offline 🛑                | Reassigning only its portion of keys               |
| A new server is added ⚙️               | Taking just a few keys from neighbors              |
| Need for horizontal scalability 🚀     | Easily adding/removing nodes dynamically           |
| Want to avoid central mapping logic 🔁 | Nodes can route things by hash, not registry       |

---

## ⚙️ How It Works (Visually)

