pub struct Stack<T> {
    value: Vec<T>,
    length:usize
}
impl<T> Stack<T> {
    pub fn new()->Self{
        Stack{
            value:Vec::new(),
            length:0
        }
    }
}
impl <T> Stack<T> {
    
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