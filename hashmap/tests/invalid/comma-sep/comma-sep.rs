use hashmap::hashmap;
use std::collections::HashMap;

fn main() {
    // using commas as separators in the macro 
    // should fail to compile
    let map: HashMap<_, _> = hashmap!{'a', 1};
}
