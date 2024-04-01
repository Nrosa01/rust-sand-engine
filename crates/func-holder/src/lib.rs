use std::vec::Vec;

pub struct FuncVec<T> {
    funclist: Vec<Box<dyn Fn(&mut T)>>,
}

impl<T> FuncVec<T> {
    fn new() -> Self {
        FuncVec { funclist: Vec::new() }
    }

    fn add<F>(&mut self, func: F)
    where
        F: Fn(&mut T) + 'static,
    {
        self.funclist.push(Box::new(func));
    }

    fn call_all(&self, data: &mut T) {
        for func in &self.funclist {
            func(data);
        }
    }

    fn call(&self, index: usize, data: &mut  T) {
        self.funclist[index](data);
    }
}
