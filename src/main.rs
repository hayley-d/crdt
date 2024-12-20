use crdt::rga::rga::RGA;

fn main() {
    let mut rga = RGA::new(1, 1);

    // Insert elements
    let s4_a = rga
        .local_insert("A".to_string(), None, None)
        .unwrap()
        .s4vector;

    // Read the state
    println!("Current RGA State: {:?}", rga.read());

    let s4_b = rga
        .local_insert("B".to_string(), Some(s4_a), None)
        .unwrap()
        .s4vector;
    // Read the state
    println!("Current RGA State: {:?}", rga.read());

    // Delete an element
    rga.local_delete(s4_a).unwrap();

    // Update an element
    rga.local_update(s4_b, "Updated B".to_string()).unwrap();

    // Read the state
    println!("Current RGA State: {:?}", rga.read());
}
