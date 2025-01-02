use b_tree::b_tree::Btree;

mod b_tree;
mod linklist;
fn main() {
    let mut tree = Btree::new();
    tree.push(1);
    tree.push(2);
    tree.push(3);
    tree.push(4);
    tree.push(5);
    tree.push(6);
    tree.push(7);
    tree.push(8);
    tree.push(9);
    tree.push(10);
    tree.push(11);
    tree.push(12);
    tree.push(13);
    tree.push(14);
    tree.push(15);
    tree.push(16);
    let a = tree.get_left(1);
    let index_is_zero = tree.get_right(0);
    let par = tree.get_par(15);
    println!("{:?}", a);
    println!("父节点： {:?}", par);
    println!("访问index为0的结果： {:?}", index_is_zero);
    println!("{:?}", tree)
}
