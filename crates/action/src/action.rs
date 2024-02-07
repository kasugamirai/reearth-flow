use std::collections::HashMap;
use std::fmt::Display;
use std::pin::Pin;

use bytes::Bytes;
use futures::Future;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use strum_macros::EnumString;
use uuid::Uuid;

use reearth_flow_workflow::graph::NodeProperty;
use reearth_flow_workflow::id::Id;
use reearth_flow_workflow::workflow::Parameter;

use crate::{attribute_keeper, file_reader, file_writer};

pub type Port = String;
pub const DEFAULT_PORT: &str = "default";
pub type ActionDataframe = HashMap<Port, Option<ActionValue>>;

#[derive(Debug, Clone)]
pub enum ActionValue {
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<ActionValue>),
    Bytes(Bytes),
    Map(HashMap<String, ActionValue>),
}

impl Default for ActionValue {
    fn default() -> Self {
        Self::String("".to_owned())
    }
}

impl Display for ActionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionValue::Bool(v) => write!(f, "{}", v),
            ActionValue::Number(v) => write!(f, "{}", v),
            ActionValue::String(v) => write!(f, "{}", v),
            ActionValue::Array(v) => write!(f, "{:?}", v),
            ActionValue::Bytes(v) => write!(f, "{:?}", v),
            ActionValue::Map(v) => write!(f, "{:?}", v),
        }
    }
}

#[derive(Serialize, Deserialize, EnumString, Debug, Clone)]
pub enum Action {
    #[strum(serialize = "fileReader")]
    FileReader,
    #[strum(serialize = "attributeKeeper")]
    AttributeKeeper,
    #[strum(serialize = "fileWriter")]
    FileWriter,
}

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub node_id: Id,
    pub node_name: String,
    pub node_property: NodeProperty,
    pub parameter: Parameter,
}

impl Default for ActionContext {
    fn default() -> Self {
        Self {
            node_id: Uuid::new_v4(),
            node_name: "".to_owned(),
            node_property: serde_json::Map::new(),
            parameter: serde_json::Map::new(),
        }
    }
}

impl ActionContext {
    pub fn new(
        node_id: Id,
        node_name: String,
        node_property: NodeProperty,
        parameter: Parameter,
    ) -> Self {
        Self {
            node_id,
            node_name,
            node_property,
            parameter,
        }
    }
}

impl Action {
    pub fn run(
        &self,
        ctx: ActionContext,
        input: Option<ActionDataframe>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ActionDataframe>> + Send + 'static>> {
        match self {
            Action::FileReader => Box::pin(file_reader::run(ctx, input)),
            Action::AttributeKeeper => Box::pin(attribute_keeper::run(ctx, input)),
            Action::FileWriter => Box::pin(file_writer::run(ctx, input)),
        }
    }
}
