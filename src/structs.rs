use std::collections::HashMap;
use std::boxed::Box;

pub enum ConnectionErrors {
  MethodNotFound,
  DeviceNotFound,
  NoResponse
}

pub trait Connection<'a> {
  fn is_open (&self) -> bool;
  fn send (&self, data: &str);
  fn get_id (&self) -> &str;
}

pub trait CommunicationMethod <'a> {
  fn name (&self) -> &str;
  fn connect (&self, id: &str) -> Result<Box<dyn Connection>, ConnectionErrors>;
  fn set_core (&mut self, core: DeviceCore<'a>);
  fn discover (&self) -> Vec<&str>;
}

pub struct DeviceCore<'a> {
  connections: Vec<Box<dyn Connection<'a>>>,
  communication_methods:  HashMap<Box<&'a str>, Box<&'a dyn CommunicationMethod<'a>>>
}

impl <'a> DeviceCore <'a> {
  pub fn add_communication_method (&mut self, method: Box<&'a dyn CommunicationMethod<'a>>) -> &mut Self {
    self.communication_methods.insert(Box::from(method.name()), method);
    self
  }

  pub fn receive (&self, data: &str) {

  }

  pub fn connect_to (&mut self, method_name: &str, id: &str) -> Result<Box<dyn Connection>, ConnectionErrors> {
    match self.communication_methods.get(&method_name) {
      Some(s) => Ok(Box::from(s.connect(id)?)),
      None => Err(ConnectionErrors::MethodNotFound) 
    }
  }
}


// test impl

struct Test <'a> {
  communication_name: String,
  device_core: Option<DeviceCore<'a>>
}

impl<'a> CommunicationMethod<'a> for Test<'a> {
  fn name (&self) -> &str {
    &self.communication_name
  }
  fn set_core(&mut self, core: DeviceCore<'a>) {
    self.device_core = Option::from(core);
  }

  fn connect (&self, id: &str) -> Result<Box<dyn Connection<'_>>, ConnectionErrors> {
    Ok(Box::from(TestConnection {

    }))
  }

  fn discover(&self) -> Vec<&str> {
    vec!["gfhjdhalidf"]
  }
}

struct TestConnection {
}

impl <'a> Connection<'a> for TestConnection {
  fn is_open (&self) -> bool {
    true
  }

  fn send (&self, data: &str) {

  }

  fn get_id (&self) -> &str{
    &"test"
  }
}