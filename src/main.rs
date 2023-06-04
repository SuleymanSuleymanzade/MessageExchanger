use std::any::Any;
use std::collections::HashMap;
use MessageExchanger::Message;

fn print_variable(value: &Box<dyn std::any::Any>) {
    if let Some(string_value) = value.downcast_ref::<String>() {
        println!("String value: {}", string_value);
    } else if let Some(int_value) = value.downcast_ref::<i32>() {
        println!("Integer value: {}", int_value);
    } else if let Some(bool_value) = value.downcast_ref::<bool>() {
        println!("{}", bool_value);
    } else if let Some(char_value) = value.downcast_ref::<char>(){
        println!("{}", char_value);
    }
    else if let Some(str_slize) = value.downcast_ref::<&str>(){
        println!("{}", str_slize);
    }
     else {
        println!("Unknown value type");
    }
}

fn main() {
    let mut my_map: HashMap<&str, Box<dyn std::any::Any>> = HashMap::new();
    my_map.insert("name", Box::new("John doe"));

    my_map.insert("name", Box::new("John doe"));
    my_map.insert("simbol", Box::new('a'));
    my_map.insert("check", Box::new(true));
    my_map.insert("other", Box::new(24));
    
    for (k, v) in my_map.into_iter(){
        print_variable(&v);
    }    
    
    let message:Message = Message::default();
    
}
