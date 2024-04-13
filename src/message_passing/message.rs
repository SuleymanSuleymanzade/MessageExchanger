use std::collections::HashMap;


pub trait MEHolder: std::fmt::Debug {
    // your trait definition here
}

pub struct Message {
    content: Box<dyn MEHolder>,
}

impl Message {
    pub fn new(content: Box<dyn MEHolder>) -> Self {
        Message { content }
    }

    pub fn get_message(&self) -> &Box<dyn MEHolder> {
        &self.content
    }
}






