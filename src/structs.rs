use std::collections::HashMap;
use std::boxed::Box;

pub trait Connection<'a> {
  //fn new () -> Self;
  fn is_open (self) -> bool;
  fn communication_method (self) -> dyn CommunicationMethod<'a>;
}


pub trait CommunicationMethod <'a> {
  fn name (&self) -> &str;
  fn connect (&self) -> dyn Connection;
  fn set_core (&mut self, core: DeviceCore<'a>);
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
}


// test impl

struct Test <'a> {
  communication_name: String,
  device_core: DeviceCore<'a>
}

impl<'a> CommunicationMethod<'a> for Test<'a> {
  fn name (&self) -> &str {
    &self.communication_name
  }
  fn set_core(&mut self, core: DeviceCore<'a>) {
    self.device_core = core;
  }

  fn connect (&self) -> TestConnection {

  }
}

struct TestConnection {
}

impl <'a> Connection<'a> for TestConnection {

}