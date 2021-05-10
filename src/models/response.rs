use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Response<T> {
    status: &'static str,
    code: u32,
    results: Vec<T>
}

impl<T> Response<T> {
    pub fn ok() -> Self {
        Response {
            status: "OK",
            code: 200,
            results: Vec::new()
        }
    }

    pub fn with_result(&mut self, value: T) {
        self.results.push(value);
    }
}

impl Response<Message> {
    pub fn msg(&mut self, content: String) {
        self.results.push(Message::with(content));
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    message: String
}

impl Message {
    pub fn with(content: String) -> Message {
        Message {
            message: content
        }
    }
}