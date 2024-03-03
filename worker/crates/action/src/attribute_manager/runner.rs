use core::result::Result;
use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use rayon::prelude::*;
use rhai::Dynamic;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::debug;

use reearth_flow_eval_expr::engine::Engine;
use reearth_flow_macros::PropertySchema;

use crate::action::{ActionContext, ActionDataframe, ActionResult, ActionRunner, ActionValue};
use crate::utils::convert_dataframe_to_scope_params;

#[derive(Serialize, Deserialize, Debug, PropertySchema)]
#[serde(rename_all = "camelCase")]
struct PropertySchema {
    operations: Vec<Operation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Operation {
    pub(crate) attribute: String,
    pub(crate) method: Method,
    pub(crate) value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum Method {
    #[serde(rename = "convert")]
    Convert,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "rename")]
    Rename,
    #[serde(rename = "remove")]
    Remove,
}

#[derive(Debug, Clone)]
pub(crate) enum Operate {
    Convert {
        expr: Option<rhai::AST>,
        attribute: String,
    },
    Create {
        expr: Option<rhai::AST>,
        attribute: String,
    },
    Rename {
        new_key: String,
        attribute: String,
    },
    Remove {
        attribute: String,
    },
}

pub(crate) struct AttributeManager;

#[async_trait::async_trait]
impl ActionRunner for AttributeManager {
    async fn run(&self, ctx: ActionContext, inputs: Option<ActionDataframe>) -> ActionResult {
        let props = PropertySchema::try_from(ctx.node_property)?;
        debug!(?props, "read");
        let inputs = inputs.ok_or(anyhow!("No Input"))?;
        let expr_engine = Arc::clone(&ctx.expr_engine);
        let params = convert_dataframe_to_scope_params(&inputs);
        let operations = convert_single_operation(props.operations, Arc::clone(&expr_engine));

        let mut output = ActionDataframe::new();
        for (port, data) in inputs {
            let data = match data {
                Some(data) => data,
                None => continue,
            };
            let value = match data {
                ActionValue::Array(rows) => {
                    // NOTE: Parallelization with a small number of cases will conversely slow down the process.
                    let processed_data = match rows.len() {
                        0..=1000 => rows
                            .iter()
                            .map(|row| mapper(row, &operations, &params, Arc::clone(&expr_engine)))
                            .collect::<Vec<_>>(),
                        _ => rows
                            .par_iter()
                            .map(|row| mapper(row, &operations, &params, Arc::clone(&expr_engine)))
                            .collect::<Vec<_>>(),
                    };
                    ActionValue::Array(processed_data)
                }
                ActionValue::Map(row) => mapper(
                    &ActionValue::Map(row),
                    &operations,
                    &params,
                    Arc::clone(&expr_engine),
                ),
                _ => data,
            };
            output.insert(port, Some(value));
        }
        Ok(output)
    }
}

fn mapper(
    row: &ActionValue,
    operations: &[Operate],
    params: &HashMap<String, ActionValue>,
    expr_engine: Arc<Engine>,
) -> ActionValue {
    match row {
        ActionValue::Map(row) => {
            let mut result = row.clone();
            for operation in operations {
                match operation {
                    Operate::Convert { expr, attribute } => {
                        let value = row.get(attribute);
                        if value.is_none() {
                            continue;
                        }
                        let scope = expr_engine.new_scope();
                        for (k, v) in params {
                            scope.set(k, v.clone().into());
                        }
                        for (k, v) in row {
                            scope.set(k, v.clone().into());
                        }
                        if let Some(expr) = expr {
                            let new_value = scope.eval_ast::<Dynamic>(expr);
                            if let Ok(new_value) = new_value {
                                if let Ok(new_value) = new_value.try_into() {
                                    result.insert(attribute.clone(), new_value);
                                }
                            }
                        }
                    }
                    Operate::Create { expr, attribute } => {
                        let scope = expr_engine.new_scope();
                        for (k, v) in params {
                            scope.set(k, v.clone().into());
                        }
                        for (k, v) in row {
                            scope.set(k, v.clone().into());
                        }
                        if let Some(expr) = expr {
                            let new_value = scope.eval_ast::<Dynamic>(expr);
                            if let Ok(new_value) = new_value {
                                if let Ok(new_value) = new_value.try_into() {
                                    result.insert(attribute.clone(), new_value);
                                }
                            }
                        }
                    }
                    Operate::Rename { new_key, attribute } => {
                        let value = row.get(attribute);
                        if value.is_none() {
                            continue;
                        }
                        result.remove(attribute);
                        result.insert(new_key.clone(), value.unwrap().clone());
                    }
                    Operate::Remove { attribute } => {
                        let value = row.get(attribute);
                        if value.is_none() {
                            continue;
                        }
                        result.remove(attribute);
                    }
                };
            }
            ActionValue::Map(result)
        }
        _ => row.clone(),
    }
}

fn convert_single_operation(operations: Vec<Operation>, expr_engine: Arc<Engine>) -> Vec<Operate> {
    operations
        .iter()
        .map(|operation| {
            let method = &operation.method;
            let attribute = &operation.attribute;
            let value = operation.value.clone().unwrap_or_default();
            match method {
                Method::Convert => Operate::Convert {
                    expr: expr_engine.compile(&value).ok(),
                    attribute: attribute.clone(),
                },
                Method::Create => Operate::Create {
                    expr: expr_engine.compile(&value).ok(),
                    attribute: attribute.clone(),
                },
                Method::Rename => Operate::Rename {
                    new_key: value,
                    attribute: attribute.clone(),
                },
                Method::Remove => Operate::Remove {
                    attribute: attribute.clone(),
                },
            }
        })
        .collect::<Vec<_>>()
}
