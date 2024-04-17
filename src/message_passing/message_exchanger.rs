use super::message::Message;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::any::type_name;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct FolderParams {
    folder: String,
    history_size: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct MessageExchangerParams {
    root: String,
    history_size: u32,
    folders_params: Vec<FolderParams>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigYaml {
    MessageExchangerParams: MessageExchangerParams,
}

pub struct Utils;
impl Utils {
    pub fn read_yaml_config(file_path: &str) -> ConfigYaml {
        let local_path = Path::new(file_path)
            .canonicalize()
            .expect("Failed to get the absolute path");

        println!("{}", local_path.display());

        let file_content = fs::read_to_string(local_path).expect("Failed to read file");
        let params: ConfigYaml =
            serde_yaml::from_str(&file_content).expect("Failed to deserialize YAML");
        return params;
    }
}

#[derive(Debug)]
pub struct MessageExchanger {
    state: ConfigYaml, //
    ops_flag: bool,    //operation flag
}

#[allow(dead_code)]
impl MessageExchanger {
    fn create_folders_struct(&self, root_path: &str, folders: &Vec<FolderParams>) {
        let last_symbol = root_path.chars().last().unwrap_or_default();
        let mut global_root_path = PathBuf::from(root_path);

        if last_symbol == '/' {
            global_root_path.push("airflow_exchange");
        }

        // Creating root dir
        fs::create_dir(&global_root_path);

        let _ = fs::create_dir(&global_root_path); // creating root dir
        for i in folders.iter() {
            let folder = &i.folder;
            let folder_path = Path::new(&&global_root_path).join(folder);
            println!("{}", folder_path.display());
            let _ = fs::create_dir(folder_path);
        }
    }

    pub fn setup(&self) {
        let root_path = &self.state.MessageExchangerParams.root;
        let folders = &self.state.MessageExchangerParams.folders_params;
        self.create_folders_struct(root_path, folders)
    }

    pub fn new(config: &str) -> Self {
        MessageExchanger {
            state: Utils::read_yaml_config(config),
            ops_flag: false,
        }
    }
}
