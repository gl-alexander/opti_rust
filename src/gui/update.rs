pub fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
}