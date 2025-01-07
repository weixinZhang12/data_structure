use std::{collections::HashMap, time::Instant, vec};

use b_tree::b_tree_link::Btree;
use hash::hash_link::hashmap;
use recursion::normal::memoized_feibo;

mod b_tree;
mod linklist;
mod recursion;
mod hash;
fn main() {
   let mut map=hashmap::new(3);
   map.put(0, "t".to_string());
   map.put(1, "t".to_string());
   map.put(2, "t".to_string());
   map.put(4, "t".to_string());
   map.put(5, "t".to_string());
   map.put(6, "t".to_string());
   map.put(8, "t".to_string());
   for i in 0..10000000 {
       map.put(i, "val".to_string());
   }
   println!("{:#?}",map);
  
}
