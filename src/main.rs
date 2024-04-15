mod message_passing {
    pub mod message;
    pub mod message_exchanger;
}

use message_passing::message::Message;
use message_passing::message_exchanger::{ConfigYaml, MessageExchanger, Utils};
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Person {
    name: String,
    surname: String,
}

fn main() {
    // let any_value = Message::new(42);

    // let mut any_value = Message::new("Hello, Rust!");
    // let res = any_value.get_content::<&str>().unwrap();
    // println!("{}", res);

    // any_value.set_param("first", "hohoho");
    // let first = any_value.get_param::<&str>("first").unwrap();
    // println!("{}", first);

    // any_value.set_param("second", 123);
    // let second = any_value.get_param::<i32>("second").unwrap();
    // println!("{}", second);

    // println!("{}", any_value.get_last_update_time());
    // println!("{:?}", any_value.get_keys());

    let util = Utils {};
    let params: ConfigYaml =
        Utils::read_yaml_config(r"C:\Users\sul_s\Documents\MessageExchanger\me_config.yaml");
    println!("{:#?}", params);

    let me = MessageExchanger::new(r"C:\Users\sul_s\Documents\MessageExchanger\me_config.yaml");
    me.setup();
}
