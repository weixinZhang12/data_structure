pub mod b_tree_link {
    use std::vec;

    #[derive(Debug)]
    pub struct BTreeNode {
        pub value: i32,
        pub left: Option<Box<BTreeNode>>,
        pub right: Option<Box<BTreeNode>>,
    }
    #[derive(Debug)]
    pub struct Btree {
        root: Option<Box<BTreeNode>>,
    }
    impl BTreeNode {
        fn new(value: i32) -> Self {
            BTreeNode {
                value,
                left: None,
                right: None,
            }
        }
    }
    impl Btree {
        pub fn new() -> Self {
            Btree { root: None }
        }
        pub fn push(&mut self, value: i32) {
            let current_node = &mut self.root as *mut Option<Box<BTreeNode>>;
            while let Some(v) = unsafe { &mut *current_node } {}
        }
        ///返回应该插入元素位置信息的数组,0为左,1为右
        pub fn index_to_lr(&self, mut index: usize) -> Vec<usize> {
            // 数据的位置为索引位置取余数为0,不包括成为0的次数，二进制的数从右到左取次数位
            index += 1;
            let mut temp: Vec<usize> = vec![];
            let mut index_t = index;
            let mut count = 0;
            while index_t / 2 != 0 {
                index_t = index_t / 2;
                count += 1;
            }
            let length = count;
            for i in 0..length {
                let t = index & 1;
                temp.push(t);
                index = index >> 1;
            }
            temp.reverse();
            temp
        }
        pub fn vec_to_tree(&mut self, arr: Vec<i32>) {
            // let mut tree = Btree::new();
            // 如果输入的是一个空的数组，直接将空树返回
            if arr.len() == 0 {
                return;
            }
            let mut current = &mut self.root as *mut Option<Box<BTreeNode>>;
            let root_ptr = current;
            for (index, num) in arr.iter().enumerate() {
                current = root_ptr;
                let new_node = BTreeNode::new(*num);
                let lr_arr = self.index_to_lr(index);
                println!("{:?}",lr_arr);
                let lr_length = lr_arr.len();
                if lr_length == 0 {
                    self.root = Some(Box::new(new_node));
                    // println!("{:?}",tree);
                    continue;
                }

                for i in lr_arr {
                    if i == 0 {
                        current = unsafe { &mut (*current).as_mut().unwrap().left }
                            as *mut Option<Box<BTreeNode>>;
                    } else if i == 1 {
                        current = unsafe { &mut (*current).as_mut().unwrap().right }
                            as *mut Option<Box<BTreeNode>>;
                    }
                }
                unsafe { *current = Some(Box::new(new_node)) }

                // println!("{:?}",tree)
            }
        }
        fn prin_t(&self, root: &Option<Box<BTreeNode>>) {
            if root.is_none() {
                return;
            }
            if let Some(v) = root {
                self.prin_t(&v.left);
                println!("{}", v.value);
                self.prin_t(&v.right);
            }
        }
        pub fn print_tree(&self) {
            self.prin_t(&self.root);
        }
        fn di_vec(&self, root: &Option<Box<BTreeNode>>, arr: &mut Vec<i32>) {
            if root.is_none() {
                return;
            }
            if let Some(v) = root {
                println!("left start");
                self.di_vec(&v.left, arr);
                println!("left end");

                arr.push(v.value);
                println!("right start");
                println!("{:?}",arr);
                self.di_vec(&v.right, arr);
                println!("right end");

            }
        }
        pub fn tree_to_vec(&self) -> Vec<i32> {
            let mut arr: Vec<i32>=vec![];
            self.di_vec(&self.root, &mut arr);
            arr
        }
        fn vis(&self,root:&Option<Box<BTreeNode>>,mut c:i32){
            c=c*2;
            if root.is_none() {
                return;
            }
            if let Some(v) = root {
                self.vis(&v.left, c);
                for i in 0..c {
                    print!("-");
                }
                print!("{}",v.value);
                println!("");
                self.vis(&v.right, c);

            }
        }
        pub fn vision(&self){
            let c=1;
            self.vis(&self.root, c);
        }
    }
}

pub mod b_tree {

    use std::error::Error;

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
        pub fn insert(&mut self, index: usize, value: i32) -> Result<(), Box<dyn Error>> {
            if index >= MAX_INDEX {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Index bound out",
                )));
            }
            self.arr[index] = Some(value);
            Ok(())
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
        ///返回只读数组。无法访问到第一个元素（索引为0的元素）
        pub fn get_iter(&self) -> &[Option<i32>] {
            &self.arr[1..]
        }
    }
}
