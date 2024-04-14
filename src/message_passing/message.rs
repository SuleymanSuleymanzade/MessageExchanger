use std::any::Any;
use std::collections::HashMap;

pub struct Message {
    content: Box<dyn Any>,
    params: HashMap<String, Box<dyn Any>>,
}

impl Message {
    pub fn new<T:Any>(data: T) -> Self{
        Message{
            content: Box::new(data),
            params: HashMap::new(),
        }
    }

    pub fn get_content<T:Any>(&mut self) -> Option<&mut T>{
        self.content.downcast_mut()
    }

    pub fn set_content<T:Any>(&mut self, value: T){
        self.content = Box::new(value);
    }

    pub fn set_param<T:Any>(&mut self, key:&str, val:T){
        self.params.insert(key.to_string(), Box::new(val));
    }

    pub fn get_param<T: Any>(&mut self, key: &str) -> Option<&mut T> {
        if let Some(par) = self.params.get_mut(key) {
            if let Some(inner) = par.downcast_mut::<T>() {
                Some(inner)
            } else {
                None
            }
        } else {
            None
        }
    }

}

