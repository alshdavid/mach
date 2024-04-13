pub struct NodejsAdapter {
  messages
}

impl NodejsAdapter {
  pub fn ping(&self) -> Result<(), String> {
    Ok(())
  }

  fn send(&self, bytes: Vec<u8>) -> Vec<u8> {

    return vec![];
  }
}