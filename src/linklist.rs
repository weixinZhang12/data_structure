#[derive(Debug)]
pub struct LinkNode<T> {
    value: T,
    next: Option<Box<LinkNode<T>>>,
}

#[derive(Debug)]
pub struct LinkList<T> {
    head: Option<Box<LinkNode<T>>>, // 修改为 Box<LinkNode<T>>
}

impl<T> LinkList<T> {
    pub fn new() -> Self {
        LinkList { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = LinkNode { value, next: None };
        // 如果头部为空那么添加到这里
        // 临时用于存储当前的指向的节点
        let mut current_node = &mut self.head;
        while let Some(v) = current_node {
            current_node = &mut v.next;
        }
        *current_node = Some(Box::new(new_node))
    }
    pub fn pop(&mut self) {
        let mut current_node = &mut self.head;
        // 如果当前节点有值
        while let Some(v) = current_node {
            // 如果当前节点的下一个节点没有值
            if let None = v.next {
                break;
            } else {
                current_node = &mut v.next;
            }
        }

        *current_node = None;
    }
}
