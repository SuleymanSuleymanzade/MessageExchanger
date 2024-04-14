mod message_passing{
    pub mod message;
}

use message_passing::message::{Message};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Person {
    name: String,
    surname: String,
}

fn main() {

    let mut p: Person = Person{
        name:"S".to_string(), 
        surname:"sn".to_string()
    };

    let any_value = Message::new(42);

    let mut any_value = Message::new("Hello, Rust!");

    let res = any_value.get_content::<&str>();
    println!("{}", res.unwrap());


    any_value.set_param("first", "hohoho");

    let res = any_value.get_param::<&str>("first")
        .unwrap();

    println!("{}", res);

}
