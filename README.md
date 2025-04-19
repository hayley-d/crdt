# Replicated Growable Array (RGA) with S4Vector

This repository provides an implementation of a **Replicated Growable Array (RGA)**, a distributed data structure for collaborative applications. The RGA ensures **eventual consistency** and conflict resolution using the **S4Vector** indexing system, a robust mechanism for deterministic operation ordering in distributed environments.


---

## What is a Replicated Growable Array (RGA)?

An **RGA** is a **Conflict-free Replicated Data Type (CRDT)** designed to maintain a sequence of elements across distributed systems. It enables multiple users to concurrently perform operations (insertions, deletions, updates) without conflicts and guarantees that all replicas converge to the same final state. This makes RGAs ideal for:

- **Collaborative text editors**: Applications like Google Docs or Microsoft Word Online.
- **Real-time coding platforms**: Tools like Replit or CodePen.
- **Shared design systems**: Collaborative design tools like Figma or Canva.


### Why Use an RGA?

1. **Concurrency Support**:
   - Users can perform operations simultaneously, and the RGA resolves conflicts deterministically.

2. **Eventual Consistency**:
   - All replicas converge to the same state, even in asynchronous networks.

3. **Lightweight Coordination**:
   - No locks or central servers are required for synchronization.

4. **Efficiency**:
   - Supports fine-grained operations on sequences, with operations resolved using efficient data structures.

5. **Resilience**:
   - Can handle out-of-order operations, dropped messages, or intermittent network failures.

---


## Key Features

- **Conflict-free Replicated Data Type (CRDT)**:
  - Handles concurrent operations without requiring centralized coordination.

- **S4Vector for Precedence and Ordering**:
  - A deterministic mechanism for resolving operation conflicts.
  - Combines session ID, logical clock, site ID, and sequence number for unique operation identifiers.

- **Operations**:
  - Insert, Delete, and Update with support for local and remote handling.

- **Buffering**:
  - Unresolved operations (e.g., those missing dependencies) are buffered until their requirements are satisfied.

- **Traversal**:
  - Provides a linear view of the RGA, skipping tombstoned (deleted) elements.

- **Broadcasting**:
  - Simulates synchronization of operations between multiple replicas.

---

## How It Works

The RGA uses **S4Vectors** as unique identifiers for nodes (elements in the sequence) and operations. These identifiers enable:

1. **Deterministic Conflict Resolution**:
   - Concurrent operations are ordered using S4Vector precedence.

2. **Causal Consistency**:
   - Operations are applied respecting their causal dependencies.

### **S4Vector**
The **S4Vector** is a structured identifier with four fields:

```rust
#[derive(Debug, Clone, Copy)]
pub struct S4Vector {
    ssn: u64,  // Session ID (global session identifier)
    sum: u64,  // Logical clock value
    sid: u64,  // Site ID (unique per replica)
    seq: u64,  // Sequence number (local clock increment)
}
```

#### How S4Vectors Are Generated

1. **Insertions**:
   - When inserting an element between two neighbors (`left` and `right`), the `sum` is computed as the average of their sums.
   - If one neighbor is absent, the `sum` is adjusted accordingly to append or prepend.

2. **Uniqueness**:
   - Combining `ssn`, `sid`, and `seq` ensures no two operations share the same S4Vector.


```rust
impl S4Vector {
    pub fn generate(
        left: Option<&S4Vector>,
        right: Option<&S4Vector>,
        current_session: u64,
        local_site: u64,
        local_sequence: &mut u64,
    ) -> Self {
        *local_sequence += 1;
        let new_sum = match (left, right) {
            (Some(l), Some(r)) => (l.sum + r.sum) / 2, // Average of neighbors
            (Some(l), None) => l.sum + 1,              // Append to the end
            (None, Some(r)) => r.sum / 2,              // Insert at start
            (None, None) => 1,                         // First element
        };
        S4Vector {
            ssn: current_session,
            sum: new_sum,
            sid: local_site,
            seq: *local_sequence,
        }
    }
}
```

### **Node**
A node represents an element in the RGA.

```rust
#[derive(Debug, Clone)]
pub struct Node {
    pub value: String,             // The actual content
    pub s4vector: S4Vector,        // Unique identifier for ordering
    pub tombstone: bool,           // Marks if the node is logically deleted
    pub left: Option<S4Vector>,    // Left neighbor
    pub right: Option<S4Vector>,   // Right neighbor
}
```

## Core Components

### **Node**
Each node represents an element in the array. It contains metadata for conflict resolution, ordering, and traversal.

```rust
#[derive(Debug, Clone)]
pub struct Node {
    pub value: String,             // The actual content
    pub s4vector: S4Vector,        // Unique identifier for ordering
    pub tombstone: bool,           // Marks if the node is logically deleted
    pub left: Option<S4Vector>,    // Left neighbor
    pub right: Option<S4Vector>,   // Right neighbor
}
```

### **Operations**
The RGA supports three primary operations:

1. **Insert**: Adds a new element between two existing elements.
2. **Delete**: Marks an element as logically deleted (tombstoned).
3. **Update**: Updates the value of an existing element (provided it isn’t tombstoned).

---

## Benefits of This Implementation

### **1. Robust Conflict Resolution**

- This implementation guarantees **deterministic resolution** of concurrent operations by leveraging the S4Vector’s ordering rules. For example, when two users insert elements at the same position, the S4Vector precedence ensures a consistent order across all replicas.

### **2. Buffering for Unresolved Operations**

- Operations referencing missing dependencies (e.g., an insert with a non-existent left neighbor) are buffered. Once the dependencies are resolved, the buffered operations are applied automatically.

### **3. Tombstone Handling**

- Deleted elements are not physically removed but marked as tombstones. This ensures causal consistency and prevents invalid references to removed elements.

### **4. Lightweight Synchronization**

- Changes are broadcast as minimal operation messages, reducing network overhead. Remote operations are applied in the same way as local ones, ensuring consistency.

---

## Usage

### Initialize the RGA

Create a new RGA instance with a session ID and a unique site ID for the replica.

```rust
let mut rga = RGA::new(session_id: 1, site_id: 1);
```

### Insert an Element

Insert a value between two nodes (or at the beginning):

```rust
let left = None;  // Insert at the beginning
let right = None;
let result = rga.local_insert("A".to_string(), left, right);

match result {
    Ok(broadcast_op) => println!("Inserted successfully: {:?}", broadcast_op),
    Err(e) => println!("Failed to insert: {:?}", e),
}
```

### Delete an Element

Mark an element as deleted using its S4Vector:

```rust
let s4 = S4Vector { ssn: 1, sum: 2, sid: 1, seq: 1 };
let result = rga.local_delete(s4);

match result {
    Ok(broadcast_op) => println!("Deleted successfully: {:?}", broadcast_op),
    Err(e) => println!("Failed to delete: {:?}", e),
}
```

### Update an Element

Modify the value of an existing element:

```rust
let s4 = S4Vector { ssn: 1, sum: 2, sid: 1, seq: 1 };
let result = rga.local_update(s4, "Updated Value".to_string());

match result {
    Ok(broadcast_op) => println!("Updated successfully: {:?}", broadcast_op),
    Err(e) => println!("Failed to update: {:?}", e),
}
```

### Read the Current State

Traverse the RGA and retrieve all non-tombstoned values:

```rust
let state = rga.read();
println!("Current RGA State: {:?}", state);
```

---

## Example Workflow

```rust
fn main() {
    let mut rga = RGA::new(1, 1);

    // Insert elements
    let s4_a = rga.local_insert("A".to_string(), None, None).unwrap().s4vector;
    let s4_b = rga.local_insert("B".to_string(), Some(s4_a), None).unwrap().s4vector;

    // Delete an element
    rga.local_delete(s4_a).unwrap();

    // Update an element
    rga.local_update(s4_b, "Updated B".to_string()).unwrap();

    // Read the state
    println!("Current RGA State: {:?}", rga.read());
}
```

**Output:**
```plaintext
Current RGA State: ["Updated B"]
```

---

## References
- Kleppmann, M., Mulligan, D. P., Gomes, V. B. F., and Beresford, A. R. (2020) Interleaving anomalies in collaborative text editors. Available at: https://api.repository.cam.ac.uk/server/api/core/bitstreams/046dab31-5c1e-4ad1-b6f8-06d41456eef3/content [Accessed 18 December 2024].

- Kleppmann, M. (2019) CRDTs: The hard parts. YouTube. Available at: https://www.youtube.com/watch?v=x7drE24geUw [Accessed 18 December 2024].

- Roh, H.-G., Jeon, M., Kim, J.-S. & Lee, J. (2011) Replicated abstract data types: Building blocks for collaborative applications. J. Parallel Distrib. Comput, 71, pp. 354-368.*



