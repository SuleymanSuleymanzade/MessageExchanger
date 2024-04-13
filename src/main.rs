use std::any::Any;
use std::collections::HashMap;
use MessageExchanger::Message;

#[derive(Debug)]
struct Person{
    name: String,
    surname: String,
}


fn main() {


    let message:Message = Message<Person>::new(
        name: "Suleyman".to_string(),
        surname: "Suleymanzade".to_string()
    );
    
}
