pub struct Stack {
    top: i32,
    store: Vec<i32>,
}
impl Stack {
    fn push(mut self, n: i32) {
        self.top = n;
        self.store.push(n);
    }
    fn pop(mut self) {
        let a = self.store.pop();
        self.top = self.store.last().unwrap().to_owned();
        a.unwrap();
    }
    fn top(self)->i32 {
        let a = self.store.last();
        *a.unwrap()
    }
    
}
