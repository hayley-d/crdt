pub mod rgs {
    use crate::S4Vector;
    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;
    #[allow(dead_code)]

    /// Node in the RGA represents an element in the array.
    /// The value is the actual content (string or character)
    /// The S4Vector is a unique identifier for the node
    /// The Tombstone is a boolean indicating if the node has been marked as deleted.
    /// The left and right pointers are references to adjacent nodes in the linked list
    #[derive(Debug, Clone)]
    pub struct Node {
        pub value: String,
        pub s4vector: S4Vector,
        pub tombstone: bool,
        pub left: Option<S4Vector>,
        pub right: Option<S4Vector>,
    }

    #[derive(Debug, Clone)]
    pub enum OperationType {
        Insert,
        Update,
        Delete,
    }

    // Represents an operation that is temporarily inresloved due to dependancies
    #[derive(Debug, Clone)]
    struct Operation {
        operation: OperationType,
        s4vector: S4Vector,
        value: Option<String>, // Can be None for delete operations
        left: Option<S4Vector>,
        right: Option<S4Vector>,
    }

    /// #Example
    /// ```
    /// fn main() {
    ///     let mut rga : RGA = RGA::new(1,1);
    ///
    ///     // Insert elements locally
    ///     let s4_1 : S4Vector = rga.local_insert("A".to_string(),None,None);
    ///     let s4_2 : S4Vector = rga.local_insert("B".to_string(),Some(s4_1),None);
    ///
    ///     // Delete element
    ///     rga.local_delete(s4_1);
    ///
    ///     // Read the RGA state
    ///     let current_state : Vec<String> = rga.read();
    ///     println!("RGA state: {:?}",current_state);
    ///
    ///     // Simulate a remote insert
    ///     let remote_s4_3 = S4Vector {
    ///         ssn:2,
    ///         sum:3,
    ///         sid:2,
    ///         seq: 1
    ///     };
    ///     rga.remote_insert(remote_s4_3,"C".to_string(),Some(s4_2),None);
    ///
    ///     // Read the Updated sate
    ///     let updated_state: Vec<String> = rga.read();
    ///     println!("{:?}",updated_state);
    ///     ```

    pub struct RGA {
        head: Option<S4Vector>,
        hash_map: HashMap<S4Vector, Rc<RefCell<Node>>>,
        buffer: VecDeque<Operation>, // Buffer to hold unresolved operations
        session_id: u64,             //Current session ID
        site_id: u64,                // Unique ID for this replica
        local_sequence: u64,         // Logical clock for this replica
    }

    #[derive(Debug, thiserror::Error)]
    pub enum OperationError {
        #[error("Failed to perform operation, dependancies have not been met")]
        DependancyError,
    }

    pub struct BroadcastOperation {
        pub operation: OperationType,
        pub s4vector: S4Vector,
        pub value: Option<String>,
        pub left: Option<S4Vector>,
        pub right: Option<S4Vector>,
    }

    impl Node {
        pub fn new(
            value: String,
            s4: S4Vector,
            left: Option<S4Vector>,
            right: Option<S4Vector>,
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

    impl RGA {
        pub fn new(session_id: u64, site_id: u64) -> Self {
            return RGA {
                head: None,
                hash_map: HashMap::new(),
                buffer: VecDeque::new(),
                session_id,
                site_id,
                local_sequence: 0,
            };
        }

        fn insert_into_list(&mut self, node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
            let left: &Option<S4Vector> = &node.borrow().left;

            if let Some(left) = left {
                let mut current: S4Vector = left.clone();
                while let Some(node) = self.hash_map.get(&current) {
                    if let Some(next_s4) = &node.borrow_mut().right {
                        if next_s4 > &node.borrow().s4vector {
                            break;
                        }
                        current = next_s4.clone();
                    } else {
                        break;
                    }
                }

                if let Some(other) = self.hash_map.get(&current) {
                    node.borrow_mut().right = other.borrow().right;
                    other.borrow_mut().right = Some(node.borrow().s4vector);
                }
            }

            if self.head.is_none() {
                self.head = Some(node.borrow().s4vector);
            } else if left.is_none() {
                self.head = Some(node.borrow().s4vector);
            }

            return Rc::clone(&node);
        }

        /// Local insert operation to insert the element into the list at a specific position.
        pub fn local_insert(
            &mut self,
            value: String,
            left: Option<S4Vector>,
            right: Option<S4Vector>,
        ) -> Result<BroadcastOperation, OperationError> {
            let new_node: Node = match (left, right) {
                (Some(l), Some(r)) => {
                    // Generate the S4Vector
                    let new_s4: S4Vector = S4Vector::generate(
                        Some(&l),
                        Some(&r),
                        self.session_id,
                        self.site_id,
                        &mut self.local_sequence,
                    );

                    // Check if the dependensies are resolved
                    if !self.hash_map.contains_key(&l) {
                        self.buffer.push_back(Operation {
                            operation: OperationType::Insert,
                            s4vector: new_s4,
                            value: Some(value),
                            left,
                            right,
                        });
                        return Err(OperationError::DependancyError);
                    }

                    Node::new(value, new_s4, Some(l), Some(r))
                }
                (Some(l), None) => {
                    let new_s4: S4Vector = S4Vector::generate(
                        Some(&l),
                        None,
                        self.session_id,
                        self.site_id,
                        &mut self.local_sequence,
                    );

                    // Check if the dependensies are resolved
                    if !self.hash_map.contains_key(&l) {
                        self.buffer.push_back(Operation {
                            operation: OperationType::Insert,
                            s4vector: new_s4,
                            value: Some(value),
                            left,
                            right,
                        });
                        return Err(OperationError::DependancyError);
                    }

                    Node::new(value, new_s4, Some(l), None)
                }
                (None, Some(r)) => {
                    let new_s4: S4Vector = S4Vector::generate(
                        None,
                        Some(&r),
                        self.session_id,
                        self.site_id,
                        &mut self.local_sequence,
                    );

                    // Check if the dependensies are resolved
                    if !self.hash_map.contains_key(&r) {
                        self.buffer.push_back(Operation {
                            operation: OperationType::Insert,
                            s4vector: new_s4,
                            value: Some(value),
                            left,
                            right,
                        });
                        return Err(OperationError::DependancyError);
                    }

                    Node::new(value, new_s4, None, Some(r))
                }
                (None, None) => {
                    let new_s4: S4Vector = S4Vector::generate(
                        None,
                        None,
                        self.session_id,
                        self.site_id,
                        &mut self.local_sequence,
                    );

                    Node::new(value, new_s4, None, None)
                }
            };
            let new_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(new_node));
            let node: Rc<RefCell<Node>> = self.insert_into_list(new_node);

            // Insert into the hash table
            self.hash_map
                .insert(node.borrow().s4vector, Rc::clone(&node));

            self.apply_buffered_operations();

            return Ok(BroadcastOperation {
                operation: OperationType::Insert,
                s4vector: node.borrow().s4vector,
                value: Some(node.borrow().value.clone()),
                left: node.borrow().left.clone(),
                right: node.borrow().right.clone(),
            });
        }

        /// Local operation to mark an element as deleted based on the given UID.
        pub fn local_delete(
            &mut self,
            s4vector: S4Vector,
        ) -> Result<BroadcastOperation, OperationError> {
            let node: Rc<RefCell<Node>> = match self.hash_map.get(&s4vector) {
                Some(node) => Rc::clone(&node),
                None => {
                    self.buffer.push_back(Operation {
                        operation: OperationType::Delete,
                        s4vector,
                        value: None,
                        left: None,
                        right: None,
                    });
                    return Err(OperationError::DependancyError);
                }
            };

            node.borrow_mut().tombstone = true;

            self.apply_buffered_operations();

            return Ok(BroadcastOperation {
                operation: OperationType::Delete,
                s4vector: node.borrow().s4vector,
                value: None,
                left: node.borrow().left.clone(),
                right: node.borrow().right.clone(),
            });
        }

        /// Local operation to modify the content of an existing element.
        pub fn local_update(
            &mut self,
            s4vector: S4Vector,
            value: String,
        ) -> Result<BroadcastOperation, OperationError> {
            let node: Rc<RefCell<Node>> = match &self.hash_map.get(&s4vector) {
                Some(node) => Rc::clone(node),
                None => {
                    self.buffer.push_back(Operation {
                        operation: OperationType::Update,
                        s4vector,
                        value: Some(value),
                        left: None,
                        right: None,
                    });
                    return Err(OperationError::DependancyError);
                }
            };
            if !node.borrow().tombstone {
                node.borrow_mut().value = value;
            }
            self.apply_buffered_operations();

            return Ok(BroadcastOperation {
                operation: OperationType::Update,
                s4vector: node.borrow().s4vector,
                value: Some(node.borrow().value.clone()),
                left: node.borrow().left.clone(),
                right: node.borrow().right.clone(),
            });
        }

        /// Remote operation to add a new element at a position based on a provided UID
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_insert(
            &mut self,
            value: String,
            s4vector: S4Vector,
            left: Option<S4Vector>,
            right: Option<S4Vector>,
        ) {
            let new_node: Node = match (left, right) {
                (Some(l), Some(r)) => Node::new(value, s4vector, Some(l), Some(r)),
                (Some(l), None) => Node::new(value, s4vector, Some(l), None),
                (None, Some(r)) => Node::new(value, s4vector, None, Some(r)),
                (None, None) => Node::new(value, s4vector, None, None),
            };
            let new_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(new_node));
            let node: Rc<RefCell<Node>> = self.insert_into_list(new_node);

            self.hash_map
                .insert(node.borrow().s4vector, Rc::clone(&node));
            self.apply_buffered_operations();
        }

        /// Remote operation to remove an ekement given the UID
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_delete(&mut self, s4vector: S4Vector) {
            let node: Rc<RefCell<Node>> = match self.hash_map.get(&s4vector) {
                Some(node) => Rc::clone(&node),
                None => {
                    // The values has not been added yet
                    return;
                }
            };
            node.borrow_mut().tombstone = true;
            self.apply_buffered_operations();
        }

        /// Remote operation to update an element
        /// This operation updates the RGA to ensure eventual consistency
        pub fn remote_update(&mut self, s4vector: S4Vector, value: String) {
            let node: Rc<RefCell<Node>> = Rc::clone(&self.hash_map[&s4vector]);
            if !node.borrow().tombstone {
                node.borrow_mut().value = value;
            }
            self.apply_buffered_operations();
        }

        /// Reads the RGA in its current state while skipping any tombstoned nodes.
        pub fn read(&self) -> Vec<String> {
            let mut result: Vec<String> = Vec::new();
            let mut current: Option<S4Vector> = self.head;

            while let Some(current_s4) = current {
                if let Some(node) = self.hash_map.get(&current_s4) {
                    if !node.borrow().tombstone {
                        result.push(node.borrow().value.clone());
                    }

                    current = node.borrow().right;
                } else {
                    break;
                }
            }
            return result;
        }

        pub fn apply_buffered_operations(&mut self) {
            let mut buffer: VecDeque<Operation> = self.buffer.clone();

            buffer.retain(|op| {
                if let Some(left) = &op.left.clone() {
                    if !self.hash_map.contains_key(left) {
                        return true;
                    }
                }

                match op.operation {
                    OperationType::Insert => {
                        if let Some(value) = &op.value {
                            self.remote_insert(value.clone(), op.s4vector, op.left, op.right);
                        }
                    }
                    OperationType::Update => {
                        if let Some(value) = &op.value {
                            self.remote_update(op.s4vector, value.to_string());
                        }
                    }
                    OperationType::Delete => {
                        self.remote_delete(op.s4vector);
                    }
                }

                false
            });

            self.buffer = buffer;
        }
    }
}
