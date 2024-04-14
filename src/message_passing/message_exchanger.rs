use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use serde_yaml;

use super::message::Message;

//Create datastructure for serialize config data
#[derive(Debug, Deserialize, Serialize)]
struct FileSystem{
    history_size: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExchangerState{
    me_type: Option<String>,
    history_size: Option<u32>,
    file_system: Option<HashMap<String, FileSystem>>,
}

pub struct Utils;
impl Utils{
    pub fn read_yaml(&self, file_path:&str){
        let file = File::open(file_path).expect("Error: Failed to open the file");
        let reader = BufReader::new(file);
        //let exchanger_state:ExchangerState = serde_yaml::from_reader(reader).expect("Failed to decerialize yaml.");
        println!("{:?}", exchanger_state);
        match serde_yaml::from_reader::<_, ExchangerState>(reader) {
            Ok(exchanger_state) => {
                println!("MessageExchanger:");
                println!("  me_type: {:?}", exchanger_state.me_type);
                
                if let Some(history_size) = exchanger_state.history_size {
                    println!("  history_size: {}", history_size);
                }

                if let Some(file_system) = &exchanger_state.file_system {
                    println!("  file_system:");
                    for (folder, fs) in file_system {
                        println!("    {}: {{", folder);
                        
                        if let Some(history_size) = fs.history_size {
                            println!("      history_size: {}", history_size);
                        }
                        
                        println!("    }}");
                    }
                }
            },
            Err(e) => println!("Failed to deserialize YAML: {}", e),
    }
}


pub struct MessageExchanger{

}
#[allow(dead_code)]
impl MessageExchanger{

    pub fn new(&self, conf_file:&str){

    }
}