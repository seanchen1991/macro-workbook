use hashmap::hashmap;
use std::collections::HashMap;

fn main() {
    // a single argument is invalid
    let map: HashMap<_, _> = hashmap!{"foo"};
}
