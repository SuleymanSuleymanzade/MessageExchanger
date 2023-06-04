use std::collections::HashMap;
use std::any::Any;

#[derive(Clone)]
pub struct Message<'a>{
    message: Option<Box<dyn Any>>,
    params: Option<HashMap<&'a str, Box<dyn Any>>>,
}

impl Clone for Message{
    fn clone(&self) -> Self {
        Self {
            data: self.message.clone(),
            params: self.params.clone()
        }
    }
}

impl<'a> Message<'a>{

    pub fn new(message: Option<Box<dyn Any>>, 
        params: Option<HashMap<&'a str, Box<dyn Any>>>) -> Self{
            // Constructor
            Message{
                message:message,
                params:params,
            }
        }

    pub fn default()->Self{
        //Constructor with default params
        Message{
            message:None,
            params:None,
        }
    }
    // setter and getters for message
    pub fn set_message(&mut self, message: Option<Box<dyn Any>>){
        self.message = message;
    }

    pub fn get_message(&self) -> Option<Box<dyn Any>>{
        self.message
    }

}