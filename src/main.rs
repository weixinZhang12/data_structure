use std::vec;

use b_tree::b_tree_link::Btree;

mod b_tree;
mod linklist;
fn main() {
    let mut tree = Btree::new();
    let mut v: Vec<i32> = vec![];
    for i in 1..20 {
        v.push(i);
    }
    tree.vec_to_tree(v);
    // tree.print_tree();
    // println!("{:#?}", tree);
    // let arr=tree.tree_to_vec();
    // println!("{:?}",arr);
    tree.vision();
    //    println!("{:?}",a)
}
