pub mod link_list {
    use colored::Colorize;

    #[derive(Debug)]
    pub struct LinkNode {
        value: i32,
        next: Option<Box<LinkNode>>,
    }

    #[derive(Debug)]
    pub struct LinkList {
        head: Option<Box<LinkNode>>, // 修改为 Box<LinkNode<i32>>
    }

    impl LinkList {
        pub fn new() -> Self {
            LinkList { head: None }
        }
        ///向链表中添加元素
        pub fn push(&mut self, value: i32) {
            let new_node = LinkNode { value, next: None };
            // 如果头部为空那么添加到这里
            // 临时用于存储当前的指向的节点
            let mut current_node = &mut self.head;
            while let Some(v) = current_node {
                current_node = &mut v.next;
            }
            *current_node = Some(Box::new(new_node))
        }
        ///删除链表中第一个符合要求的元素
        pub fn delete(&mut self, value: i32) {
            let mut current_node = &mut self.head as *mut Option<Box<LinkNode>>;
            let mut last_node: *mut Option<Box<LinkNode>> =
                &mut self.head as *mut Option<Box<LinkNode>>;
            unsafe {
                while let Some(current_some) = &mut *current_node {
                    if current_some.value == value {
                        if let Some(last_some) = &mut *last_node {
                            if current_node == last_node {
                                self.head = current_some.next.take();
                                return;
                            }
                            last_some.next = (*current_node).take().unwrap().next;
                            return;
                        }
                    } else {
                        last_node = current_node;
                        current_node = &mut current_some.next as *mut Option<Box<LinkNode>>;
                    }
                }
            }
            println!(
                "{} {}",
                "Not found this value: ".yellow().bold(),
                value.to_string().red().bold()
            )
        }
        ///弹出最后一个元素
        pub fn pop(&mut self) -> Option<i32> {
            let mut current_node = &mut self.head as *mut Option<Box<LinkNode>>;
            unsafe {
                while let Some(v) = &mut *current_node {
                    if v.next.is_none() {
                        return Some((&mut *current_node).take().unwrap().value);
                    } else {
                        current_node = &mut v.next;
                    }
                }
            }
            None
        }
        pub fn take_head(&mut self) {
            if let Some(v) = &self.head {
                self.delete(v.value);
            }
        }
    }
}
