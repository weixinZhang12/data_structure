pub mod b_tree_link{
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
        pub fn push(&mut self,value: i32) {
            let current_node=&mut self.root as * mut Option<Box<BTreeNode>>;
            while let Some(v) = unsafe { &mut *current_node } {
                
            }
        }
    }
    
}

pub mod b_tree{
    #[derive(Debug)]
    pub struct Btree{
        arr:[i32;10]
    }
    impl Btree {
        pub fn new()->Self{
            Btree{
                arr:[0;10]
            }
        }
        pub fn push(&mut self,value:i32){
            for (index,&arr_value) in self.arr.iter().enumerate() {
                if self.arr[index]==0 {
                    self.arr[index]=value;
                    return;
                }
            }
        }
    }
}