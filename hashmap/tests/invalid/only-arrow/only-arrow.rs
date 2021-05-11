use hashmap::hashmap;
use std::collections::HashMap;

fn main() {
    // a single `=>` is invalid
    let map: HashMap<_, _> = hashmap!{=>};
}
