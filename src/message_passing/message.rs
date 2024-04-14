use std::any::Any;
use std::collections::HashMap;
use chrono::{Local, DateTime};

pub struct Message {
    content: Box<dyn Any>,
    params: HashMap<String, Box<dyn Any>>,
    last_update:DateTime<Local>,
}

impl Message {
    pub fn new<T:Any>(data: T) -> Self{
        Message{
            content: Box::new(data),
            params: HashMap::new(),
            last_update:Local::now(),
        }
    }

    pub fn get_content<T:Any>(&mut self) -> Option<&mut T>{
        self.content.downcast_mut()
    }

    pub fn set_content<T:Any>(&mut self, value: T){
        self.content = Box::new(value);
        self.last_update = Local::now();
    }

    pub fn set_param<T:Any>(&mut self, key:&str, val:T){
        self.params.insert(key.to_string(), Box::new(val));
        self.last_update = Local::now();
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
    
    pub fn get_keys(&mut self) -> Vec<String>{
        let keys = self.params
            .keys()
            .cloned()
            .collect();
        return keys 
    }

    pub fn get_last_update_time(&self) -> String{
        self.last_update.format("%Y-%m-%d %H:%M:%S").to_string()
    }

}

