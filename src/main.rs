use b_tree::b_tree::Btree;

mod b_tree;
mod linklist;
fn main() {
    let mut tree = Btree::new();
    tree.push(1);
    tree.push(2);
    tree.push(2);
    tree.push(2);
 
    let left = tree.get_left(0);
    let index_is_zero = tree.get_right(0);
    let par = tree.get_partent(2);
    println!("0的左子节点：{:?}", left);
    println!("2的父节点： {:?}", par);
    println!("访问index为0的结果： {:?}", index_is_zero);
    println!("{:?}", tree);
    // 只读迭代器遍历
    for item in tree.get_iter() {
        if let Some(v) = item {
         println!("{}",v)
        }
    }
}
