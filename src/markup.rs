use crate::Message;
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "messages.html")]
pub struct MessagesTemplate {
    pub(crate) messages: Vec<Message>,
}

#[derive(Template)]
#[template(path = "message_item.html")]
pub struct MessageItemTemplate {
    pub(crate) message: Message,
}
