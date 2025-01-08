pub mod hash_link {
    #[derive(Clone, Debug)]
    pub struct hash_node {
        pub key: usize,
        pub val: String,
    }
    #[derive(Clone, Debug)]

    pub struct hashmap {
        buckets: Vec<Option<hash_node>>,
        max_len: usize,
        len: usize,
    }
    impl hash_node {
        fn new(key: usize, val: String) -> Self {
            Self { key, val }
        }
    }
    impl hashmap {
        ///传入一个大小参数
        pub fn new(max_len: usize) -> Self {
            Self {
                buckets: vec![None; max_len],
                max_len,
                len: 0,
            }
        }
        /// 计算出应该插入的位置
        pub fn hash(&self, key: usize) -> usize {
            key % self.max_len
        }
        // 插入数据,直接覆盖模式
        pub fn insert(&mut self, key: usize, val: String) {
            let hash = self.hash(key);
            self.buckets[hash] = Some(hash_node::new(key, val));
        }
        ///如果原地址已经存在了，寻找新的地址，该方法为递归方法，有栈溢出的风险
        fn find_new_index(&self, key: usize, first_index: usize) -> Option<usize> {
            let mut key = key;
            // 递归结束条件
            if self.buckets[key].is_some() {
                key = self.hash(key + 1);
                // 递归结束条件
                if key == first_index {
                    return None;
                }
                // 此处hash函数多余
                self.find_new_index(self.hash(key), first_index)
            } else {
                Some(key)
            }
        }
        // 寻找新的位置，该方法位迭代方法
        fn find_new_key(&self, key: usize) -> Option<usize> {
            let mut key = self.hash(key);
            let mut first = self.hash(key);
            while self.buckets[key].is_some() {
                key = self.hash(key + 1);
                if key == first {
                    return None;
                }
            }
            Some(key)
        }
        pub fn get(&mut self, key: usize) -> Option<hash_node> {
            let mut hash = self.hash(key);
            let first_key = self.hash(key);
            while self.buckets[hash].is_some() {
                if self.buckets[hash].as_ref().unwrap().key == key {
                    return self.buckets[hash].clone();
                } else {
                    hash = self.hash(hash + 1);
                    if hash == first_key {
                        return None;
                    }
                }
            }
            None
        }
        pub fn delete(&mut self, key: usize) -> Option<hash_node> {
            let mut hash = self.hash(key);
            let first_key = self.hash(key);
            while self.buckets[hash].is_some() {
                if self.buckets[hash].as_ref().unwrap().key == key {
                    return self.buckets[hash].take();
                } else {
                    hash = self.hash(hash + 1);
                    if hash == first_key {
                        return None;
                    }
                }
            }
            None
        }
        ///插入数据，当散列表空间不足的时候自动增加空间
        pub fn put(&mut self, key: usize, val: String) {
            let mut key = key;
            let hash = self.hash(key);
            if let Some(v) = &self.buckets[hash] {
                // 键重复的时候需要提前结束
                if v.key == key {
                    return;
                }
                let new_index = self.find_new_key(key);
                // 如果找不到空位置证明已经满了
                if new_index.is_none() {
                    self.resize(self.max_len * 2);
                    self.put(key, val);
                } else {
                    // 否则直接插入
                    self.buckets[new_index.unwrap()] = Some(hash_node::new(key, val));
                    self.len += 1;
                }
            } else {
                self.buckets[hash] = Some(hash_node::new(key, val));
                self.len += 1;
            }
        }
        // 重新分配哈希的大小
        pub fn resize(&mut self, len: usize) {
            self.max_len = len;
            // 创建一个新的数组
            let mut arr: Vec<Option<hash_node>> = vec![None; len];
            // 将原有的数据移动到新的数组
            for bucket in self.buckets.iter_mut() {
                if let Some(v) = bucket.take() {
                    let key = v.key % len;
                    arr[key] = Some(v);
                }
            }
            self.buckets = arr;
        }
    }
}
