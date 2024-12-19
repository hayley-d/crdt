#[derive(Debug, Clone)]
/// #Example
/// ```
/// let current_session : u64 = 1; // Current collaboration session ID
/// let local_site : u64 = 42; // Unique replica ID
/// let mut local_sequence : u64 = 0; // logical clock for the site
/// // Generate a base S4Vector
/// let s4_1 : S4Vector = S4Vector::generate(None,None,current_session,local_site,&mut
/// local_sequence);
/// println!("S4Vector 1: {:?}",s4_1);
///
/// // Generate a second S4Vector, simulating an insert after s4_1
/// let s4_2 : S4Vector = S4Vector::generate(Some(s4_1),None,current_session,local_site,&mut
/// local_sequence);
/// println!("S4Vector 2: {:?}",s4_2);
///
/// //Compare the two S4Vectors
/// if s4_1 < s4_2 {
///     println!("S4Vector 1 precedes S4Vector 2");
/// } else {
///     println!("S4Vector 2 precedes S4Vector 1");
/// }
/// ```
pub struct S4Vector {
    /// Session ID: Identifies the collaboration session (ensures global uniqueness)
    ssn: u64,
    /// Sum: The cumulative logical clock
    sum: u64,
    /// Site ID: Identifies the replica generating the operation.
    sid: u64,
    /// Sequence Number: Tracks the logical order of operations within the same site.
    seq: u64,
}

impl std::hash::Hash for S4Vector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ssn.hash(state);
        self.sum.hash(state);
        self.sid.hash(state);
        self.seq.hash(state);
    }
}

/// Ensures two vectors are equal only iff all fields match
impl PartialEq for S4Vector {
    fn eq(&self, other: &Self) -> bool {
        return self.ssn == other.ssn
            && self.sum == other.sum
            && self.sid == other.sid
            && self.seq == other.seq;
    }
}

/// Ensures two vectors are equal only iff all fields match
impl Eq for S4Vector {}

/// Determines the total ordering using ssn,sum,sid and seq.
impl PartialOrd for S4Vector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

/// Determines the total ordering using ssn,sum,sid and seq.
impl Ord for S4Vector {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self
            .ssn
            .cmp(&other.ssn)
            .then(self.sum.cmp(&other.sum))
            .then(self.sid.cmp(&other.sid))
            .then(self.seq.cmp(&other.seq));
    }
}

impl S4Vector {
    /// Generates a new S4Vector based on neighboring nodes and the local sequence.
    pub fn generate(
        left: Option<&S4Vector>,
        right: Option<&S4Vector>,
        current_session: u64,
        local_site: u64,
        local_sequence: &mut u64,
    ) -> Self {
        *local_sequence += 1;

        let new_sum = match (left, right) {
            (Some(l), Some(r)) => (l.sum + r.sum) / 2, // average
            (Some(l), None) => l.sum + 1,              // append to the end
            (None, Some(r)) => r.sum / 2,              // Insert at start
            (None, None) => 1,                         // first element
        };

        return S4Vector {
            ssn: current_session,
            sum: new_sum,
            sid: local_site,
            seq: *local_sequence,
        };
    }
}
