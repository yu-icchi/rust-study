use std::collections::BTreeMap;
use std::collections::Bound::{Included, Unbounded};

fn main() {
    let mut tree = BTreeMap::new();
    tree.insert(1, "value01");
    tree.insert(2, "value02");
    tree.insert(3, "value03");
    tree.insert(4, "value04");
    tree.insert(5, "value05");
    match tree.get(&2) {
        Some(review) => println!("{}", review),
        None => println!("none"),
    }
    for (&key, &value) in &tree.range(Included(&3), Included(&5)) {
        println!("{}: {}", key, value);
    }
}
