use linklist::LinkList;

mod linklist;
fn main() {
let mut a=LinkList::new();
a.push(12);
a.push(23232312);
a.pop();
a.pop();
a.pop();
println!("{:?}",a);
}