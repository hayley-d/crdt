pub mod rgs {
    use crate::S4Vector;
    use std::collections::{HashSet, LinkedList};
    #[allow(dead_code)]
    use std::fmt::Display;

    /// Node in the RGA represents an element in the array.
    /// The value is the actual content (string or character)
    /// The S4Vector is a unique identifier for the node
    /// The Tombstone is a boolean indicating if the node has been marked as deleted.
    /// The left and right pointers are references to adjacent nodes in the linked list
    pub struct Node {
        value: String,
        s4vector: S4Vector,
        tombstone: bool,
        left: Box<Node>,
        right: Box<Node>,
    }

    /// Clock keeps the logical time for the replica, it increments for every operation that
    /// occurs.
    struct Clock {
        count: u64,
    }

    pub struct RGA {
        nodes: LinkedList<Node>,
        hash_map: HashSet<S4Vector, Node>,
        current_session: u64,
        local_site: u64,
        local_sequence: u64,
    }

    impl Node {
        pub fn new(value: String, s4: S4Vector, left: Box<Node>, right: Box<Node>) -> Self {
            return Node {
                value,
                s4vector: s4,
                tombstone: false,
                left,
                right,
            };
        }
    }

    impl RGA {
        pub fn new() -> Self {
            return RGA {
                nodes: LinkedList::new(),
                hash_map: hm,
            };
        }

        fn insert_into_list(node: Node, left: Option<Box<Node>>, right: Option<Box<Node>>) {
            todo!()
        }
        /// Local insert operation to insert the element into the list at a specific position.
        pub fn local_insert(
            &mut self,
            value: String,
            left: Option<Box<Node>>,
            right: Option<Box<Node>>,
        ) {
            match (left, right) {
                (Some(l), Some(r)) => {
                    let new_s4: S4Vector = S4Vector::generate(Some(&l.s4vector), Some(&r.s4vector));
                    let new_node: Node = Node::new(value, new_s4, l, r);
                    RGA::insert_into_list(new_node, Some(l), Some(r));
                }
                (Some(l), None) => {
                    let new_s4: S4Vector = S4Vector::generate_s4vector(&l.s4vector, None);
                    let new_node: Node = Node::new(value, new_s4, left, right);
                    RGA::insert_into_list(node, left, right);
                }
                (None, Some(r)) => {
                    let new_s4: S4Vector = RGA::generate_s4vector(&left.s4vector, &right.s4vector);
                    let new_node: Node = Node::new(value, new_s4, left, right);
                    RGA::insert_into_list(node, left, right);
                }
                (None, None) => {
                    let new_s4: S4Vector = RGA::generate_s4vector(&left.s4vector, &right.s4vector);
                    let new_node: Node = Node::new(value, new_s4, left, right);
                    RGA::insert_into_list(node, left, right);
                }
            }

            todo!()
        }

        /// Local operation to mark an element as deleted based on the given UID.
        pub fn delete(&mut self, uid: Uid) {
            todo!()
        }

        /// Local operation to modify the content of an existing element.
        pub fn update(&mut self, uid: Uid) {
            todo!()
        }

        /// Remote operation to add a new element at a position based on a provided UID
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_insert(&mut self, value: String, uid: Uid) {}

        /// Remote operation to remove an ekement given the UID
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_delete(&mut self, uid: Uid) {
            todo!()
        }

        /// Remote operation to update an element
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_update(&mut self, uid: Uid) {
            todo!()
        }
    }

    impl PartialEq for Uid {
        fn eq(&self, other: &Self) -> bool {
            return self.logical_clock == other.logical_clock
                && self.fractional_position == other.fractional_position;
        }
    }

    impl PartialOrd for Uid {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.fractional_position == other.fractional_position {
                return self.logical_clock.partial_cmp(&other.logical_clock);
            } else if self.fractional_position > other.fractional_position {
                return Some(std::cmp::Ordering::Greater);
            } else {
                return Some(std::cmp::Ordering::Less);
            }
        }
    }

    impl Eq for Uid {}

    impl Ord for Uid {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self.fractional_position == other.fractional_position {
                return self.logical_clock.cmp(&other.logical_clock);
            } else if self.fractional_position > other.fractional_position {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Less;
            }
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

    impl Display for Uid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}:{})", self.logical_clock, self.fractional_position)
        }
    }
}
