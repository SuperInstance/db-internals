# db-internals

**Learn database internals by reading Rust.**

B-trees. Write-ahead logging. ARIES recovery. Two-phase locking. Query optimization. All the stuff inside every database, implemented in clean Rust you can actually read.

~3,200 lines of source. 73 tests. Zero unsafe code.

---

## What's inside

| Module | What it does |
|---|---|
| `relational` | Relations, tuples, and the full relational algebra — select, project, join, union, difference, rename |
| `btree` | B-tree with insert, search, range queries, and node splitting |
| `hash_index` | Chained hash table (separate chaining) and extendible hash table (dynamic hashing) |
| `query_opt` | Query optimizer with cost estimation, selection pushdown, and DP-based join ordering |
| `transactions` | ACID transaction schedules, conflict detection, precedence graphs, conflict serializability |
| `concurrency` | Two-phase locking (2PL) with shared/exclusive locks, lock upgrade, deadlock detection |
| `recovery` | Write-ahead logging (WAL), ARIES-style redo/undo recovery, checkpoints, CLRs |
| `normal_forms` | Functional dependencies, attribute closure, 1NF/2NF/3NF/BCNF, lossless decomposition |

---

## Install

```toml
[dependencies]
db-internals = "0.1"
```

---

## Build a mini database in 20 lines

```rust
use db_internals::*;

// Create a table
let mut users = Relation::new(vec!["id".into(), "name".into(), "age".into()]);
users.insert(Tuple::new(vec![Value::Int(1), Value::Text("Alice".into()), Value::Int(30)]));
users.insert(Tuple::new(vec![Value::Int(2), Value::Text("Bob".into()), Value::Int(25)]));

// Query it — find everyone aged 30
let results = users.select(|t, schema| {
    let idx = schema.iter().position(|s| s == "age").unwrap();
    t.values[idx] == Value::Int(30)
});

// Index it with a B-tree
let mut index = BTree::with_order(4);
index.insert(1, b"Alice".to_vec());
index.insert(2, b"Bob".to_vec());
assert_eq!(index.search(&1), Some(&b"Alice".to_vec()));

// Run transactions and check serializability
let schedule = Schedule::new(vec![
    Operation::read(1, "x"),
    Operation::write(2, "x"),
]);
assert!(schedule.precedence_graph().is_acyclic());

// Log writes for crash recovery
let mut wal = WAL::new();
wal.append(LogRecord::Begin { txn: 1 });
wal.append(LogRecord::Update {
    lsn: 0, txn: 1, page_id: 0, offset: 0,
    before: b"old".to_vec(), after: b"new".to_vec(),
});
wal.append(LogRecord::Commit { txn: 1 });
```

That's a table, an index, a transaction schedule, and a write-ahead log. This is what databases do.

---

## How it works

**Relational algebra** — The five fundamental operations (σ, π, ⋈, ∪, −) are complete: any SQL query is a composition of these.

**B-tree** — Balanced tree where every leaf is at the same depth. O(log n) search, insert, and range queries. Nodes split when full.

**Hash indexes** — Chained hashing for simplicity. Extendible hashing for auto-resizing with O(1) average lookup.

**Query optimizer** — Estimates plan costs using statistics, then uses dynamic programming to find the optimal join order. Selection pushdown moves filters close to the data source.

**ACID & serializability** — Builds a precedence graph from a transaction schedule. If it's acyclic, the schedule is conflict serializable.

**Two-phase locking** — Growing phase (acquire locks) → shrinking phase (release locks). Guarantees serializability. Can deadlock.

**ARIES recovery** — Analysis → Redo → Undo. Replays committed work, rolls back uncommitted transactions. The WAL rule ensures log records hit disk before data pages.

**Normal forms** — Functional dependencies → attribute closure → candidate keys → BCNF decomposition. Lossless, dependency-preserving where possible.

---

## License

MIT OR Apache-2.0
