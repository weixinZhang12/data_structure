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
   map.put(0, "0".to_string());
   map.put(1, "1".to_string());
   map.put(2, "2".to_string());
   map.put(4, "3".to_string());
   map.put(5, "4".to_string());
   map.put(6, "5".to_string());
   map.put(8, "6".to_string());
   for i in 0..15 {
       map.put(i, "val".to_string());
   }
   println!("{:?}",map.get(1));
   println!("{:?}",map.delete(1));
   println!("{:?}",map.delete(1));
   println!("{:?}",map.delete(14));
   println!("{:?}",map.delete(15));
   println!("{:#?}",map);
  
}
