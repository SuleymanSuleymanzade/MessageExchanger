use std::collections::HashMap;

pub trait Holder{}

pub struct Message<T: Holder, Copy>{
    buff: T 
}

impl Message{
    fn new(&self, data) -> Self{
        self.buff = data;
    }
}






