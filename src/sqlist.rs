use colored::Colorize;

#[derive(Debug)]
pub struct Sqlite<T> {
    length: usize,
    pub max_length: usize,
    value: Vec<T>,
}
impl<T> Sqlite<T> {
    pub fn new() -> Self {
        Sqlite {
            length: 0,
            max_length: 0,
            value: Vec::new(),
        }
    }
}
impl<T> Sqlite<T> {
    /// 检查索引是否超出边界
    fn is_index_out_bound(&self, index: usize) -> bool {
        if index >= self.length {
            println!("{}", "索引发生越界,元素不会被删除或添加".red());
            return true;
        } else {
            return false;
        }
    }
    ///检查线性表是否已满
    fn is_full(&self) -> bool {
        if self.max_length == 0 {
            return false;
        }
        //length不可以大于等于索引
        if self.length >= self.max_length {
            println!("{}", "数组已满,元素不会被添加".red());
            return true;
        } else {
            return false;
        }
    }
    ///向线性表中添加元素
    pub fn push(&mut self, value: T) {
        if self.is_full() {
            return;
        }
        self.value.push(value);
        self.length += 1
    }
    ///通过索引删除元素
    pub fn delete_by_index(&mut self, index: usize) {
        if self.is_index_out_bound(index) {
            return;
        }
        self.value.remove(index);
        self.length -= 1
    }
    pub fn size(&self)->usize{
        return self.length;
    }
    pub fn is_empty(&self)->bool{
    if self.length!=0 {
        false
    }
    else {
        true
    }
    }
}
