mod message_passing{
    pub mod message;
}

use message_passing::message::{Message, MEHolder};

#[derive(Debug, Clone)]
struct Person{
    name: String,
    surname: String,
}

impl Holder for Person{} // important trait

fn main() {
    let message:Message<Person> = Message::new(
        Person{
            name: "Suleyman".to_string(),
            surname: "Suleymanzade".to_string()
        }
    );
    
    let data = message.get_message();
    println!("{:?}",data);
}
