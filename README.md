# Replicated Growable Array (RGA) with S4Vector

This repository provides an implementation of a **Replicated Growable Array (RGA)**, a distributed data structure for collaborative applications. The RGA supports concurrent operations like inserts, deletes, and updates, while ensuring **eventual consistency** and conflict resolution using the **S4Vector** indexing system.

---

## What is a Replicated Growable Array (RGA)?

An **RGA** is a **Conflict-free Replicated Data Type (CRDT)** designed for maintaining a sequence of elements in distributed systems. Unlike traditional data structures, RGAs are built to handle **concurrent modifications** without requiring centralized coordination or locking mechanisms. This makes them ideal for **collaborative applications** such as:

- Real-time text editors (e.g., Google Docs, Microsoft Word Online).
- Collaborative coding platforms (e.g., Replit, CodePen).
- Multi-user design tools (e.g., Figma, Canva).

### Benefits of Using an RGA
1. **Eventual Consistency**:
   - Changes made to the array across multiple replicas are guaranteed to converge to the same state.

2. **Concurrency Support**:
   - Allows multiple users to perform operations simultaneously without conflicts.

3. **No Centralized Coordination**:
   - Operates in distributed systems without requiring locks or a central server.

4. **Efficient Conflict Resolution**:
   - Uses deterministic rules (via S4Vector) to resolve concurrent operations seamlessly.

5. **Resilience to Failures**:
   - Handles out-of-order operations through buffering and retries.

6. **Versatility**:
   - Supports operations like insert, delete, and update, making it suitable for a wide range of collaborative use cases.

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

### **S4Vector**
The **S4Vector** is a unique identifier for operations and nodes in the RGA. It ensures:

- **Deterministic conflict resolution**: Concurrent operations are ordered based on their S4Vector.
- **Eventual consistency**: Operations are always applied in the correct order across all replicas.

#### S4Vector Structure
```rust
#[derive(Debug, Clone, Copy)]
pub struct S4Vector {
    ssn: u64,  // Session ID
    sum: u64,  // Logical clock
    sid: u64,  // Site ID (replica identifier)
    seq: u64,  // Sequence number
}
```

#### S4Vector Generation
To generate a new S4Vector:
- Combine the `left` and `right` neighbors to calculate the `sum`.
- Use the current session ID and local replica’s logical clock for uniqueness.

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

### **Operations**
The RGA supports three core operations:

#### Insert
Inserts a new node between two existing nodes.

- **Inputs**: Value, `left` neighbor, `right` neighbor.
- **Outputs**: A broadcast operation to notify other replicas.

#### Delete
Marks an element as deleted by setting its tombstone to `true`.

- **Inputs**: The S4Vector of the node to delete.
- **Outputs**: A broadcast operation.

#### Update
Updates the value of an existing node, provided it isn’t tombstoned.

- **Inputs**: The S4Vector of the node and the new value.
- **Outputs**: A broadcast operation.

---

## Implementation Details

### Initialization
Create a new RGA instance for a session and replica:

```rust
let mut rga = RGA::new(session_id: 1, site_id: 1);
```

### Insert
```rust
let left = None;  // Insert at the beginning
let right = None;
let result = rga.local_insert("A".to_string(), left, right);

match result {
    Ok(broadcast_op) => println!("Inserted successfully: {:?}", broadcast_op),
    Err(e) => println!("Failed to insert: {:?}", e),
}
```

### Delete
```rust
let s4 = S4Vector { ssn: 1, sum: 2, sid: 1, seq: 1 };
let result = rga.local_delete(s4);

match result {
    Ok(broadcast_op) => println!("Deleted successfully: {:?}", broadcast_op),
    Err(e) => println!("Failed to delete: {:?}", e),
}
```

### Update
```rust
let s4 = S4Vector { ssn: 1, sum: 2, sid: 1, seq: 1 };
let result = rga.local_update(s4, "Updated Value".to_string());

match result {
    Ok(broadcast_op) => println!("Updated successfully: {:?}", broadcast_op),
    Err(e) => println!("Failed to update: {:?}", e),
}
```

### Read
Traverse the RGA and retrieve all non-tombstoned values in order.

```rust
let state = rga.read();
println!("Current RGA State: {:?}", state);
```

---

## Why Choose This RGA Implementation?

1. **Strong Consistency Guarantees**:
   - Operations are resolved deterministically using S4Vector precedence rules.

2. **Flexible Buffering System**:
   - Handles out-of-order operations and resolves dependencies seamlessly.

3. **Efficient Traversal**:
   - Supports real-time reads by skipping tombstoned nodes.

4. **Broadcast-Ready**:
   - Easily integrates with a networking layer for multi-replica synchronization.

5. **Scalable and Robust**:
   - Designed for distributed systems with multiple concurrent users.

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



