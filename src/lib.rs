// Original Copyright © 2021 lemonxah
// Modified Copyright © 2022 stringhandler
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#[macro_use]
extern crate anyhow;

mod group;
mod target;
#[macro_use]
mod node;
mod engine;
mod workers;

pub use engine::*;
pub use group::*;
pub use node::*;
pub use target::*;
pub use workers::*;

#[cfg(test)]
mod tests {
    use crate::engine::Engine;
    use crate::workers::WorkersBuilder;
    use crate::{node::*, Worker};
    use anyhow::Result;
    use std::collections::HashMap;

    #[test]
    fn multiply_works() {
        let json_data = r#"
    {
      "id": "demo@0.1.1",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": 2
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "num",
                "data": {}
              }, {
                "node": 4,
                "input": "num2",
                "data": {}
              }, {
                "node": 5,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [-60, 182],
          "name": "Number"
        },
        "2": {
          "id": 2,
          "data": {
            "num": 3
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [-106, 378],
          "name": "Number"
        },
        "3": {
          "id": 3,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 2,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 4,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [241, 240],
          "name": "Add"
        },
        "4": {
          "id": 4,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 3,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 5,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [552.5, 204],
          "name": "Add"
        },
        "5": {
          "id": 5,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [826.5, 292],
          "name": "Multiply"
        }
      },
      "comments": []
    }
    "#;

        let mut workers = WorkersBuilder::default();
        workers.add(Number).add(Add).add(Multiply);

        let engine = Engine::<()>::new("demo@0.1.1".to_string(), workers.build());
        let nodes = engine.parse_json(json_data).unwrap();
        let nn = nodes;
        let output = engine.process(&(), &nn, 1);
        let oo = output.unwrap();
        let result = &oo["num"];
        assert_eq!(result, &OutputValue::I64(14i64));
    }

    #[test]
    fn add_works() {
        let json_data = r#"
    {
      "id": "demo@0.1.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": 2
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "num",
                "data": {}
              }, {
                "node": 4,
                "input": "num2",
                "data": {}
              }, {
                "node": 5,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [-98, 218],
          "name": "Number"
        },
        "2": {
          "id": 2,
          "data": {
            "num": 1
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [-147, 406],
          "name": "Number"
        },
        "3": {
          "id": 3,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 2,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 4,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [424, 238],
          "name": "Add"
        },
        "4": {
          "id": 4,
          "data": {
            "num2": 5
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 3,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 5,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [807.5, 228],
          "name": "Add"
        },
        "5": {
          "id": 5,
          "data": {
            "num2": 0
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [1084.5, 243],
          "name": "Add"
        }
      },
      "comments": []
    }
    "#;

        let mut workers = WorkersBuilder::default();

        workers.add(Number);
        workers.add(Add);

        let engine = Engine::new("demo@0.1.0".to_string(), workers.build());
        let nodes = engine.parse_json(json_data).unwrap();
        let output = engine.process(&(), &nodes, 1);
        let oo = output.unwrap();
        let result = &oo["num"];
        assert_eq!(result, &OutputValue::I64(7i64));
    }

    #[test]
    fn errors_propegate() {
        let json_data = r#"
    {
      "id": "demo@0.1.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": "abc"
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "num",
                "data": {}
              }, {
                "node": 4,
                "input": "num2",
                "data": {}
              }, {
                "node": 5,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [-98, 218],
          "name": "Number"
        },
        "2": {
          "id": 2,
          "data": {
            "num": 1
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [-147, 406],
          "name": "Number"
        },
        "3": {
          "id": 3,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 2,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 4,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [424, 238],
          "name": "Add"
        },
        "4": {
          "id": 4,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 3,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 5,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [807.5, 228],
          "name": "Add"
        },
        "5": {
          "id": 5,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [1084.5, 243],
          "name": "Add"
        }
      },
      "comments": []
    }
    "#;

        let mut workers = WorkersBuilder::default();

        workers.add(Number);
        workers.add(Add);

        let engine = Engine::new("demo@0.1.0".to_string(), workers.build());
        let nodes = engine.parse_json(json_data).unwrap();
        let output = engine.process(&(), &nodes, 1);

        assert!(output.err().is_some());
    }

    struct Number;
    impl Worker<()> for Number {
        fn name(&self) -> &str {
            "Number"
        }

        fn work(
            &self,
            _context: &(),
            node: &Node,
            _input_data: HashMap<String, OutputValue>,
        ) -> Result<HashMap<String, OutputValue>> {
            let result: i64 = node.get_data("num")?.unwrap();
            let mut h = HashMap::new();
            h.insert("num".to_string(), OutputValue::I64(result));
            Ok(h)
        }
    }

    struct Add;
    impl Worker<()> for Add {
        fn name(&self) -> &str {
            "Add"
        }

        fn work(
            &self,
            _context: &(),
            _node: &Node,
            input_data: HashMap<String, OutputValue>,
        ) -> Result<HashMap<String, OutputValue>> {
            let num = input_data
                .get("num")
                .ok_or_else(|| anyhow!("Missing input: num"))?;
            let num2 = input_data
                .get("num2")
                .ok_or_else(|| anyhow!("Missing input: num2"))?;

            let mut h = HashMap::new();
            h.insert(
                "num".to_string(),
                OutputValue::I64(num.as_i64()? + num2.as_i64()?),
            );
            Ok(h)
        }
    }

    struct Multiply;
    impl Worker<()> for Multiply {
        fn name(&self) -> &str {
            "Multiply"
        }

        fn work(
            &self,
            _context: &(),
            _node: &Node,
            input_data: HashMap<String, OutputValue>,
        ) -> Result<HashMap<String, OutputValue>> {
            let num = input_data
                .get("num")
                .ok_or_else(|| anyhow!("Missing input: num"))?;
            let num2 = input_data
                .get("num2")
                .ok_or_else(|| anyhow!("Missing input: num2"))?;

            let mut h = HashMap::new();
            h.insert(
                "num".to_string(),
                OutputValue::I64(num.as_i64()? * num2.as_i64()?),
            );
            Ok(h)
        }
    }
}
