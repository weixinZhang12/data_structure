use b_tree::b_tree::Btree;

mod b_tree;
mod linklist;
fn main() {
    let mut tree = Btree::new();
    tree.push(1);
    tree.push(2);
 
    let a = tree.get_left(1);
    let index_is_zero = tree.get_right(0);
    let par = tree.get_partent(1);
    println!("{:?}", a);
    println!("父节点： {:?}", par);
    println!("访问index为0的结果： {:?}", index_is_zero);
    println!("{:?}", tree)
}
