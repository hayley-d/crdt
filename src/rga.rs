pub mod rgs {
    use std::fmt::Display;

    /// The UID is a combination of the logical clock and a fractional value.
    /// This approach ensures total order of operations within the replica and the fraction
    /// determines the element's position in the array
    struct Uid {
        logical_clock: u64,
        fractional_position: f64,
    }

    /// An element of the Replicated growable array (RGA) represents a char in the text document.
    /// The value is the char being represented, the unique identifier (UID) is a tuple of the
    /// logical clock and the fractional component and the tombstone is a flag to mark for logical
    /// deletion.
    struct Element {
        value: char,
        uid: Uid,
        tombstone: bool,
    }

    /// Generates a new UID given the predecessor and successor element's fractional components
    fn generate_uid(predecessor: f64, successor: f64) -> Uid {
        let fractional_position: f64 = (predecessor + successor) / 2.0;
        return Uid {
            logical_clock: get_current_clock(),
            fractional_position,
        };
    }

    /// Gets the current logical clock count.
    fn get_current_clock() -> u64 {
        todo!()
    }

    impl Element {
        pub fn new(value: char, pred: f64, succ: f64) -> Self {
            return Element {
                value,
                uid: generate_uid(pred, succ),
                tombstone: false,
            };
        }
    }

    impl Display for Uid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}:{})", self.logical_clock, self.fractional_position)
        }
    }
}
