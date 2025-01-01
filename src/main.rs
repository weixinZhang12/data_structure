use b_tree::b_tree::Btree;

mod b_tree;
mod linklist;
fn main() {
    let mut tree = Btree::new();
    tree.push(1);
    tree.push(2);
    tree.push(3);
    println!("{:?}", tree)
}
