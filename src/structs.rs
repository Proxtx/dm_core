use std::collections::HashMap;
use std::boxed::Box;

trait Connection {
  fn is_open (self) -> bool;
  fn communication_method (self) -> dyn CommunicationMethod;
}


pub trait CommunicationMethod {
  fn name (self) -> String;
  fn connect (self) -> dyn Connection;
  fn set_core (&mut self, core: &DeviceCore);
}


pub struct DeviceCore {
  connections: Vec<Box<dyn Connection>>,
  communication_methods:  HashMap<String, Box<dyn CommunicationMethod>>
}

impl DeviceCore {
  pub fn add_communication_method (&mut self, method: Box<dyn CommunicationMethod>) -> &mut DeviceCore {
    self.communication_methods.insert(String::from(*method.name()), method);
    self
  }
}