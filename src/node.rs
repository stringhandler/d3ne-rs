use std::rc::Rc;
use std::any::{Any, TypeId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::target::{Inputs, Outputs};
use std::collections::HashMap;

#[derive(Debug)]
pub struct IOData {
  pub data: Box<dyn Any>
}

#[allow(dead_code)]
impl IOData {
  pub fn is<B: Any>(&self) -> bool {
    return TypeId::of::<B>() == (*self.data).type_id()
  }
  pub fn get<A>(&self) -> Option<&A> where A: 'static {
    return self.data.downcast_ref::<A>();
  }
}

#[macro_export ]macro_rules! iodata {
  ($data: expr) => {
    IOData {
      data: Box::new($data)
    }
  };
}

#[allow(dead_code)]
pub type OutputData = Rc<HashMap<String, IOData>>;
#[allow(dead_code)]
pub type InputData = HashMap<String, Rc<HashMap<String, IOData>>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
  pub id: i64,
  pub name: String,
  pub data: Value,
  pub group: Option<i64>,
  pub position: Vec<f32>,
  pub inputs: Inputs,
  pub outputs: Outputs
}

impl Node {
  pub fn get_number_field(&self, field: &str, inputs: &InputData) -> i64 {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
    v1.or(self.data.get(field).map(|n| n.as_i64().unwrap())).unwrap()
  }
  
  pub fn get_float_number_field(&self, field: &str, inputs: &InputData) -> f64 {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| *v.get::<f64>().unwrap()).unwrap());
    v1.or(self.data.get(field).map(|n| n.as_f64().unwrap())).unwrap()
  }
  
  pub fn get_str_field<'a>(&'a self, field: &str, inputs: &'a InputData) -> &'a str {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| *v.get::<&str>().unwrap()).unwrap());
    v1.or(self.data.get(field).map(|n| n.as_str().unwrap())).unwrap()
  }
  
  pub fn get_json_field(&self, field: &str, inputs: &InputData) -> Value {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| (v.get::<Value>()).unwrap().clone()).unwrap());
    v1.or(self.data.get(field).map(|n| serde_json::from_str(n.as_str().unwrap()).unwrap())).unwrap()
  }

  pub fn get_as_json_field(&self, field: &str, inputs: &InputData) -> Value {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| (v.get::<Value>().unwrap()).clone()).unwrap());
    v1.or(self.data.get(field).map(|v| v.clone())).unwrap()
    }

}