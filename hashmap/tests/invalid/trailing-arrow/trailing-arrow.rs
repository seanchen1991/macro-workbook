use hashmap::hashmap;
use std::collections::HashMap;

fn main() {
    // a trailing `=>` is invalid
    let map: HashMap<_, _> = hashmap!{1 => 'a', 2 =>};
}
