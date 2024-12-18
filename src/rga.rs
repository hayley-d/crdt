pub mod rgs {
    #[allow(dead_code)]
    use std::fmt::Display;

    /// The UID is a combination of the logical clock and a fractional value.
    /// This approach ensures total order of operations within the replica and the fraction
    /// determines the element's position in the array
    #[derive(Debug, Clone, Copy)]
    pub struct Uid {
        logical_clock: u64,
        fractional_position: f64,
    }

    /// An element of the Replicated growable array (RGA) represents a char in the text document.
    /// The value is the string being represented, the unique identifier (UID) is a tuple of the
    /// logical clock and the fractional component and the tombstone is a flag to mark for logical
    /// deletion.
    pub struct Element {
        value: String,
        uid: Uid,
        tombstone: bool,
    }

    /// Clock keeps the logical time for the replica, it increments for every operation that
    /// occurs.
    struct Clock {
        count: u64,
    }

    pub struct RGA {
        elements: Vec<Element>,
        /// Logical clock for the replica
        clock: Clock,
    }

    impl Element {}

    impl RGA {
        pub fn new() -> Self {
            return RGA {
                elements: Vec::new(),
                clock: Clock::default(),
            };
        }

        /// Generates a UID for a new element by retrieving the clock count and calculating the
        /// fractional position.
        fn generate_uid(&mut self, predecessor: f64, successor: f64) -> Uid {
            let fractional_position: f64 = (predecessor + successor) / 2.0;
            return Uid {
                logical_clock: self.clock.increment(),
                fractional_position,
            };
        }

        pub fn insert(&mut self, value: String, pred: Option<Uid>, succ: Option<Uid>) -> Uid {
            let uid: Uid = match (pred, succ) {
                (Some(p), Some(s)) => {
                    self.generate_uid(p.fractional_position, s.fractional_position)
                }
                (Some(p), None) => self.generate_uid(p.fractional_position, 1.0),
                (None, Some(s)) => self.generate_uid(0.0, s.fractional_position),
                (None, None) => self.generate_uid(0.0, 1.0),
            };

            let element: Element = Element {
                value,
                uid,
                tombstone: false,
            };

            self.elements.push(element);

            self.elements.sort_by(|a, b| a.uid.cmp(&b.uid));

            return uid;
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
