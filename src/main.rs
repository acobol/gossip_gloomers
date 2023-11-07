use std::io::{stdin, stdout, StdoutLock, Write};
use crate::protocol::{Message, MessageBody, Payload};

mod protocol;

struct Node {
  node_id: String,
  message_id: u64,
}

impl Node {
  pub fn handle(&mut self, input: Message, output: &mut StdoutLock) {
    match input.body.payload {
      Payload::Init { node_id, .. } => {
        self.node_id = node_id;
        let reply = Message {
          src: self.node_id.clone(),
          dest: input.src,
          body: MessageBody {
            msg_id: Some(self.message_id),
            in_reply_to: input.body.msg_id,
            payload: Payload::InitOk,
          },
        };
        self.message_id += 1;
        let _ = serde_json::to_writer(&mut *output, &reply);
        let _ = output.write_all(b"\n");
      }
      Payload::InitOk => {}
      Payload::Error { .. } => {}
      Payload::Echo { echo } => {
        let reply = Message {
          src: self.node_id.clone(),
          dest: input.src,
          body: MessageBody {
            msg_id: Some(self.message_id),
            in_reply_to: input.body.msg_id,
            payload: Payload::EchoOk { echo },
          },
        };
        self.message_id += 1;
        let _ = serde_json::to_writer(&mut *output, &reply);
        let _ = output.write_all(b"\n");
      }
      Payload::EchoOk { .. } => {}
    }
  }
}

fn main() {
  let stdin = stdin().lock();
  let mut stdout = stdout().lock();
  let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
  let mut node = Node { node_id: "default".to_string(), message_id: 0 };
  for input in inputs {
    match input {
      Ok(message) => {
        node.handle(message, &mut stdout);
      }
      Err(_) => {
        println!("Error deserializando el mensaje");
      }
    }
  }
}