use std::collections::HashMap;

pub trait Holder{}

pub struct Message<T: Holder + Clone>{
    buff: T 
}

impl<T: Holder + Clone>  Message <T>{
    pub fn new(data: T) -> Self{
        Self{
            buff: data.clone(),
        }
    }

    pub fn get_message(&self) -> T{
        self.buff.clone()
    }
}






