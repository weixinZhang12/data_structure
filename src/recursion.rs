pub mod normal {
    use std::collections::HashMap;

    pub fn feibo(value: u32) -> u32 {
        if value == 0 {
            return 0;
        } else if value == 1 {
            return 1;
        }
        let mut t1 = feibo(value - 1);
        let mut t2 = feibo(value - 2);
        // println!("{}",t1+t2);
        return t1 + t2;
    }
    pub fn memoized_feibo(value: u128, map:&mut  HashMap<u128, u128>) -> u128 {
        if value == 0 {
            return 0;
        } else if value == 1 {
            return 1;
        }
        let mut t1 = 0;
        let mut t2 = 0;
        if let Some(&v) = map.get(&(value - 1)) {
            t1 = v
        } else {
            t1 = memoized_feibo(value - 1,map);
            map.insert(value-1, t1);

        }
        if let Some(&v) = map.get(&(value - 2)) {
            t2 = v
        } else {
            t2 = memoized_feibo(value - 2,map);
            map.insert(value-2, t2);
        }
        // println!("{}",t1+t2);
        return t1 + t2;
    }
}
