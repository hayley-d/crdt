pub mod rgs {
    use crate::S4Vector;
    use std::collections::{HashMap, LinkedList};
    #[allow(dead_code)]

    /// Node in the RGA represents an element in the array.
    /// The value is the actual content (string or character)
    /// The S4Vector is a unique identifier for the node
    /// The Tombstone is a boolean indicating if the node has been marked as deleted.
    /// The left and right pointers are references to adjacent nodes in the linked list
    #[derive(Debug, Clone)]
    pub struct Node {
        value: String,
        s4vector: S4Vector,
        tombstone: bool,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    /// Clock keeps the logical time for the replica, it increments for every operation that
    /// occurs.
    struct Clock {
        count: u64,
    }

    pub struct RGA<'a> {
        nodes: LinkedList<Node>,
        hash_map: HashMap<S4Vector, &'a Node>,
        current_session: u64,
        local_site: u64,
        local_sequence: u64,
    }

    impl Node {
        pub fn new(
            value: String,
            s4: S4Vector,
            left: Option<Box<Node>>,
            right: Option<Box<Node>>,
        ) -> Self {
            return Node {
                value,
                s4vector: s4,
                tombstone: false,
                left,
                right,
            };
        }
    }

    impl std::hash::Hash for Node {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
            self.s4vector.hash(state);
            self.tombstone.hash(state);
            self.left.hash(state);
            self.right.hash(state);
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            return self.value == other.value
                && self.s4vector == other.s4vector
                && self.tombstone == other.tombstone
                && self.left == other.left
                && self.right == other.right;
        }
    }

    impl Eq for Node {}

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(&other))
        }
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            return self.s4vector.cmp(&other.s4vector);
        }
    }

    impl<'a> RGA<'a> {
        pub fn new(current_session: u64, local_site: u64) -> Self {
            return RGA {
                nodes: LinkedList::new(),
                hash_map: HashMap::new(),
                current_session,
                local_site,
                local_sequence: 0,
            };
        }

        fn insert_into_list(node: Node) -> &'a Node {
            todo!()
        }

        /// Local insert operation to insert the element into the list at a specific position.
        pub fn local_insert(
            &mut self,
            value: String,
            left: Option<Box<Node>>,
            right: Option<Box<Node>>,
        ) {
            let new_node: Node = match (left, right) {
                (Some(l), Some(r)) => {
                    let new_s4: S4Vector = S4Vector::generate(
                        Some(&l.s4vector),
                        Some(&r.s4vector),
                        self.current_session,
                        self.local_site,
                        &mut self.local_sequence,
                    );
                    Node::new(value, new_s4, Some(l), Some(r))
                }
                (Some(l), None) => {
                    let new_s4: S4Vector = S4Vector::generate(
                        Some(&l.s4vector),
                        None,
                        self.current_session,
                        self.local_site,
                        &mut self.local_sequence,
                    );
                    Node::new(value, new_s4, Some(l), None)
                }
                (None, Some(r)) => {
                    let new_s4: S4Vector = S4Vector::generate(
                        None,
                        Some(&r.s4vector),
                        self.current_session,
                        self.local_site,
                        &mut self.local_sequence,
                    );
                    Node::new(value, new_s4, None, Some(r))
                }
                (None, None) => {
                    let new_s4: S4Vector = S4Vector::generate(
                        None,
                        None,
                        self.current_session,
                        self.local_site,
                        &mut self.local_sequence,
                    );
                    Node::new(value, new_s4, None, None)
                }
            };
            let node = RGA::insert_into_list(new_node);

            self.hash_map.insert(node.s4vector, node);
            // Broadcast("INSERT",node.s4vector,value,left.s4vector,right.s4vector);
        }

        /// Local operation to mark an element as deleted based on the given UID.
        pub fn delete(&mut self, s4vector: S4Vector) {
            todo!()
        }

        /// Local operation to modify the content of an existing element.
        pub fn update(&mut self, s4vector: S4Vector) {
            todo!()
        }

        /// Remote operation to add a new element at a position based on a provided UID
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_insert(&mut self, value: String, s4vector: S4Vector) {}

        /// Remote operation to remove an ekement given the UID
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_delete(&mut self, s4vector: S4Vector) {
            todo!()
        }

        /// Remote operation to update an element
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_update(&mut self, s4vector: S4Vector) {
            todo!()
        }
    }

    impl Default for Clock {
        /// Creates a new clock
        fn default() -> Self {
            return Clock { count: 0 };
        }
    }

    impl Clock {
        /// Creates a new clock.
        pub fn new() -> Self {
            return Clock { count: 0 };
        }

        /// Increments the count of the clock.
        /// Called when a new operation is recieved by the replica.
        pub fn increment(&mut self) -> u64 {
            self.count += 1;
            self.count
        }
    }
}
