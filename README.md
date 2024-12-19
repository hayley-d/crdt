# crdt
A rust implementation of  the RGA CRDT algorithm for real-time text editing over replicas

## Summary 
### Replicated Growth Array
- **Purpose**: A data structure for collaborative applications supporting insert,update and delete operations while ensuring **eventual** consistency across distributed replicas.

- **Internal Representation**: RGAs use a linked list structure to manage ordered items, combined with tombstones for deleted elements.


### S4Vector Scheme
The S4vector (session,site,sum,sequence) is a mechanism designed to:
- Identify operations uniquely: Each operation has a globally unique vector.
- Order of operations: The S4Vector defines a total order for operations using:
 * Session number: Ensures uniqueness across different collaboration sessions.
 * Sum: Total logical time across all sites.
 * Site ID: Breaks ties between operations from different sites.
 * Sequence Number: Logical time for  a specific site.

**How S4Vectors solve challenges**
- Insert positioning: Instead of integer indices, S4vectors are used to identify where to insert elements by referencing adjacent nodes.
- COnflict Resolution: Precedence between operations determined using the vector order,ensuring deterministic resolution.

**Insert Operation**
When a new element is inserted:
1. The left and right elements are identified.
2. A new node is created with the S4Vector index
3. It is inserted into the linked list while preserving the order determined by the S4Vector.

**Delete Operation**
When an element is marked as a tombstone it ensures that the references to deleted elements are still valid for other operations.

**Update Operation**
Updates replace the value of an element. Updates on tombstones are ignored to maintain consistency

A hash table is used to map S4Vector indicies to nodes to allow for efficient lookup for remote operations.
Multiple concurrent inserts at the same position are sorted using their S4Vector indices with ties broken by precedence rules stated.


### TODOs
1. Documentation for S4Vector module
2. Tests for S4Vector module


