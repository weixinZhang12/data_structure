pub mod b_tree_link {
    pub struct BTreeNode {
        pub value: i32,
        pub left: Option<Box<BTreeNode>>,
        pub right: Option<Box<BTreeNode>>,
    }

    pub struct Btree {
        root: Option<Box<BTreeNode>>,
    }

    impl Btree {
        pub fn new() -> Self {
            Btree { root: None }
        }
        pub fn push(&mut self, value: i32) {
            let current_node = &mut self.root as *mut Option<Box<BTreeNode>>;
            while let Some(v) = unsafe { &mut *current_node } {}
        }
    }
}

pub mod b_tree {

    use colored::Colorize;
    ///最大索引，实际空间少一个单位。
    const MAX_INDEX: usize = 15;
    #[derive(Debug)]
    pub struct Btree {
        arr: [Option<i32>; MAX_INDEX],
    }
    impl Btree {
        pub fn new() -> Self {
            println!(
                "{}",
                "提示：欢迎使用树结构，树结构默认索引从1开始".green().bold()
            );
            Btree {
                arr: [None; MAX_INDEX],
            }
        }
        fn index_check(&self, index: usize) -> bool {
            if index >= MAX_INDEX {
                return true;
            }
            false
        }
        ///添加元素。如果超出范围，默认放弃此次操作
        pub fn push(&mut self, value: i32) {
            self.arr[0] = Some(0);
            for (index, &arr_value) in self.arr.iter().enumerate() {
                if self.arr[index] == None {
                    self.arr[index] = Some(value);
                    return;
                }
            }
            eprintln!(
                "{}",
                format!(
                    "错误：索引超出范围，最大容量为{}，无法添加元素",
                    MAX_INDEX - 1
                )
                .red()
                .bold()
            )
        }
        pub fn get_partent(&mut self, index: usize) -> Option<i32> {
            if index >= MAX_INDEX || index == 0 || index == 1 {
                if index == 0 {
                    eprintln!(
                        "{}",
                        format!("警告：无效的访问，你不可以访问索引为0的索引")
                            .yellow()
                            .bold()
                    )
                }
                if index >= MAX_INDEX {
                    eprintln!(
                        "{}",
                        format!(
                            "错误：你不可以访问大于容量的索引，当前的最大索引为{}，而你提供的为{}",
                            MAX_INDEX - 1,
                            index
                        )
                        .red()
                        .bold()
                    )
                }
                return None;
            }

            self.arr[index / 2]
        }
        ///获取的索引从1开始。如果尝试0索引时返回None，如果超出范围，返回None
        pub fn get_left(&mut self, index: usize) -> Option<i32> {
            if index >= MAX_INDEX || index == 0 {
                if index == 0 {
                    eprintln!(
                        "{}",
                        format!("警告：操作被拒绝，你不能访问index为0的元素，它是受到保护的")
                            .yellow()
                            .bold()
                    )
                }
                return None;
            }
            let target_index: usize = index * 2;
            if target_index < MAX_INDEX {
                return self.arr[target_index];
            }
            None
        }
        pub fn get_right(&mut self, index: usize) -> Option<i32> {
            if index >= MAX_INDEX || index == 0 {
                if index == 0 {
                    eprintln!(
                        "{}",
                        format!("警告：操作被拒绝，你不能访问index为0的元素，它是受到保护的")
                            .yellow()
                            .bold()
                    )
                }
                return None;
            }
            let target_index: usize = index * 2 + 1;
            if target_index < MAX_INDEX {
                return self.arr[target_index];
            }
            None
        }
        ///返回只读迭代器。无法访问到第一个元素（索引为0的元素）
        pub fn get_iter(&self) -> &[Option<i32>] {
            &self.arr[1..]
        }
    }
}
