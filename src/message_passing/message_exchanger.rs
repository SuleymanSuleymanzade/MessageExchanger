use super::message::Message;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
struct ExchangerState{
    global_history_size:u32,
    folder_name: String,
    file_structure:HashMap<String, u32>
}

pub struct MessageExchanger{

}

impl MessageExchanger{

    pub fn new(&self, conf_file:&str){

    }
}