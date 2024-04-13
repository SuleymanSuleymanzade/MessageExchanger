mod message_passing{
    pub mod message;
}

use message_passing::message::{Message, MEHolder};


#[derive(Debug, Clone)]
struct Person {
    name: String,
    surname: String,
}

impl MEHolder for Person {}


fn main() {
    let message = Message::new(
        Box::new(Person {
            name: "Suleyman".to_string(),
            surname: "Suleymanzade".to_string(),
        })
    );


}
