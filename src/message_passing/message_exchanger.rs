use std::any::type_name;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use serde_yaml;

use super::message::Message;

#[derive(Debug, Deserialize, Serialize)]
struct FolderParams {
    folder: String,
    history_size: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct MessageExchangerParams {
    me_type: String,
    history_size: u32,
    folders_params: Vec<FolderParams>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigYaml {
    MessageExchangerParams: MessageExchangerParams,
}

pub struct Utils;
impl Utils {
    pub fn read_yaml_config(&self, file_path: &str) -> ConfigYaml {
        let local_path = Path::new(file_path)
            .canonicalize()
            .expect("Failed to get the absolute path");

        println!("{}", local_path.display());

        let file_content = fs::read_to_string(local_path).expect("Failed to read file");

        // Deserialize YAML content
        let params: ConfigYaml =
            serde_yaml::from_str(&file_content).expect("Failed to deserialize YAML");

        return params;
    }
}

pub struct MessageExchanger {}
#[allow(dead_code)]
impl MessageExchanger {
    pub fn new(&self, conf_file: &str) {}
}
