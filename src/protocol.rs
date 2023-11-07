use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde (rename_all = "snake_case")]
pub enum Payload {
  Init{node_id: String, node_ids: Vec<String>},
  InitOk,
  Error{code: u64, text: String},
  Echo{echo: String},
  EchoOk{echo: String}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBody {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub msg_id: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub in_reply_to: Option<u64>,
  #[serde(flatten)]
  pub payload: Payload
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
  pub src: String,
  pub dest: String,
  pub body: MessageBody
}