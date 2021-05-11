use hashmap::hashmap;
use std::collections::HashMap;

fn main() {
    // an arg should be present between each pair of commas 
    let map: HashMap<_, _> = hashmap!{1 => 'a', , 2 => 'b'};
}
