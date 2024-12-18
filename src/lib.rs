pub struct Node {
    /// The charcter in the text document
    value: char,
    /// The Unique identifier for ordering across replicas
    uid: f64,
    /// A falg indicating whether a node is logically deleted
    deleted: bool,
}

pub fn compare_uids(a: f64, b: f64) -> bool {
    todo!()
}
