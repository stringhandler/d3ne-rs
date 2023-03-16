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
use crate::workers::Workers;
use crate::{node::*, WorkerError};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Version mismatch: Engine({0}), Nodes({1})")]
    VersionMismatch(String, String),
    #[error(transparent)]
    WorkerError(WorkerError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("Missing output: {node_id} {output_name}")]
    MissingOutput { node_id: i64, output_name: String },
    #[error("Invalid output type: {expected} != {actual}")]
    InvalidOutputType { expected: String, actual: String },
}

pub struct Engine<TContext> {
    id: String,
    workers: Workers<TContext>,
}

#[allow(dead_code)]
impl<TContext> Engine<TContext> {
    pub fn new(id: String, workers: Workers<TContext>) -> Self {
        Self { id, workers }
    }

    pub fn parse_json(&self, json: &str) -> Result<HashMap<i64, Node>> {
        let value: Value = serde_json::from_str(json)?;
        self.parse_value(value)
    }

    pub fn parse_value(&self, value: Value) -> Result<HashMap<i64, Node>> {
        let version = value["id"]
            .as_str()
            .ok_or(anyhow!("Engine has no version"))?
            .to_string();
        if self.id != version {
            bail!(EngineError::VersionMismatch(self.id.to_string(), version));
        }
        let nodes: HashMap<String, Node> = serde_json::from_value(value["nodes"].clone())?;
        nodes
            .into_iter()
            .map(|(k, v)| Ok((k.parse::<i64>()?, v)))
            .collect::<Result<HashMap<_, _>>>()
    }

    /// Consumes engine
    pub fn process(
        self,
        context: &TContext,
        nodes: &HashMap<i64, Node>,
        start_node_id: i64,
    ) -> Result<HashMap<String, OutputValue>> {
        let mut cache = HashMap::new();
        let mut closed_nodes: Vec<i64> = Vec::new();
        let end_id = self.process_nodes(
            context,
            &nodes[&start_node_id],
            nodes,
            &mut cache,
            &mut closed_nodes,
        )?;
        Ok((*cache[&end_id]).clone())
    }

    fn process_node(
        &self,
        context: &TContext,
        node: &Node,
        nodes: &HashMap<i64, Node>,
        cache: &mut HashMap<i64, Rc<HashMap<String, OutputValue>>>,
        closed_nodes: &mut Vec<i64>,
    ) -> Result<Rc<HashMap<String, OutputValue>>, EngineError> {
        if cache.contains_key(&node.id) {
            return Ok(cache[&node.id].clone());
        }
        if closed_nodes.contains(&node.id) {
            return Ok(Rc::new(HashMap::new()));
        }

        let mut input_data: HashMap<String, OutputValue> = HashMap::new();
        for (name, input) in &node.inputs {
            for conn in &input.connections {
                if !closed_nodes.contains(&conn.node) {
                    let out =
                        self.process_node(context, &nodes[&conn.node], nodes, cache, closed_nodes)?;
                    input_data.insert(
                        name.clone(),
                        out.get(&conn.output)
                            .ok_or_else(|| EngineError::MissingOutput {
                                node_id: conn.node,
                                output_name: conn.output.clone(),
                            })?
                            .clone(),
                    );
                    // if !out.clone().contains_key(&conn.output) && conn.output != "action" {
                    //     Self::disable_node_tree(&nodes[&conn.node], nodes, closed_nodes);
                    //     Self::disable_node_tree(node, nodes, closed_nodes);
                    // }
                }
            }
        }
        let mut output = Rc::new(HashMap::new());
        if !closed_nodes.contains(&node.id) {
            output = Rc::new(self.workers.call(&node.name, context, node, input_data)?);
            cache.insert(node.id, output.clone());
        }
        Ok(output)
    }

    fn process_nodes(
        &self,
        context: &TContext,
        node: &Node,
        nodes: &HashMap<i64, Node>,
        cache: &mut HashMap<i64, Rc<HashMap<String, OutputValue>>>,
        closed_nodes: &mut Vec<i64>,
    ) -> Result<i64, EngineError> {
        let mut id: i64 = node.id;
        if !closed_nodes.contains(&node.id) {
            let outputdata = self.process_node(context, node, nodes, cache, closed_nodes)?;
            for (name, output) in node.outputs.clone() {
                if outputdata.contains_key(&name) {
                    for connection in &output.connections {
                        if !closed_nodes.contains(&connection.node) {
                            id = self.process_nodes(
                                context,
                                &nodes[&connection.node],
                                nodes,
                                cache,
                                closed_nodes,
                            )?;
                        }
                    }
                } else if name != "action" {
                    for connection in &output.connections {
                        if connection.input == name.clone()
                            && !closed_nodes.contains(&connection.node)
                        {
                            Self::disable_node_tree(&nodes[&connection.node], nodes, closed_nodes);
                        }
                    }
                }
            }
        }
        Ok(id)
    }

    fn disable_node_tree(node: &'_ Node, nodes: &HashMap<i64, Node>, closed_nodes: &mut Vec<i64>) {
        match node.inputs.clone().get("action") {
            None => (),
            Some(input) => {
                if input.connections.len() == 1 {
                    if !closed_nodes.contains(&node.id) {
                        closed_nodes.push(node.id);
                    }
                    for output in node.outputs.clone().values() {
                        for connection in &output.connections {
                            let _node = &nodes[&connection.node];
                            if let Some(input) = _node.inputs.clone().get("action") {
                                if input
                                    .connections
                                    .clone()
                                    .into_iter()
                                    .any(|c| c.node == connection.node)
                                {
                                    Self::disable_node_tree(
                                        &nodes[&connection.node],
                                        nodes,
                                        closed_nodes,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
