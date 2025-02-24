mod citygml;
mod csv;

use std::{collections::HashMap, sync::Arc};

use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::Expr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors;
use super::errors::FeatureProcessorError;

#[derive(Debug, Clone, Default)]
pub struct FeatureReaderFactory;

impl ProcessorFactory for FeatureReaderFactory {
    fn name(&self) -> &str {
        "FeatureReader"
    }

    fn description(&self) -> &str {
        "Filters features based on conditions"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(FeatureReaderParam))
    }

    fn categories(&self) -> &[&'static str] {
        &["Feature"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn build(
        &self,
        ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let params: FeatureReaderParam = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                FeatureProcessorError::FilterFactory(format!("Failed to serialize with: {}", e))
            })?;
            serde_json::from_value(value).map_err(|e| {
                FeatureProcessorError::FilterFactory(format!("Failed to deserialize with: {}", e))
            })?
        } else {
            return Err(FeatureProcessorError::FilterFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };

        let expr_engine = Arc::clone(&ctx.expr_engine);
        match params {
            FeatureReaderParam::CityGML { common_property } => {
                let common_property = CompiledCommonPropertySchema {
                    expr: expr_engine
                        .compile(common_property.dataset.as_ref())
                        .map_err(|e| FeatureProcessorError::FilterFactory(format!("{:?}", e)))?,
                };
                let process = FeatureReader {
                    params: CompiledFeatureReaderParam::CityGML { common_property },
                };
                Ok(Box::new(process))
            }
            FeatureReaderParam::Csv {
                common_property,
                property,
            } => {
                let common_property = CompiledCommonPropertySchema {
                    expr: expr_engine
                        .compile(common_property.dataset.as_ref())
                        .map_err(|e| FeatureProcessorError::FilterFactory(format!("{:?}", e)))?,
                };
                let process = FeatureReader {
                    params: CompiledFeatureReaderParam::Csv {
                        common_property,
                        property,
                    },
                };
                Ok(Box::new(process))
            }
            FeatureReaderParam::Tsv {
                common_property,
                property,
            } => {
                let common_property = CompiledCommonPropertySchema {
                    expr: expr_engine
                        .compile(common_property.dataset.as_ref())
                        .map_err(|e| FeatureProcessorError::FilterFactory(format!("{:?}", e)))?,
                };
                let process = FeatureReader {
                    params: CompiledFeatureReaderParam::Tsv {
                        common_property,
                        property,
                    },
                };
                Ok(Box::new(process))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeatureReader {
    params: CompiledFeatureReaderParam,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommonPropertySchema {
    pub(super) dataset: Expr,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(tag = "format")]
pub enum FeatureReaderParam {
    #[serde(rename = "citygml")]
    CityGML {
        #[serde(flatten)]
        common_property: CommonPropertySchema,
    },
    #[serde(rename = "csv")]
    Csv {
        #[serde(flatten)]
        common_property: CommonPropertySchema,
        #[serde(flatten)]
        property: csv::CsvPropertySchema,
    },
    #[serde(rename = "tsv")]
    Tsv {
        #[serde(flatten)]
        common_property: CommonPropertySchema,
        #[serde(flatten)]
        property: csv::CsvPropertySchema,
    },
}

#[derive(Debug, Clone)]
enum CompiledFeatureReaderParam {
    CityGML {
        common_property: CompiledCommonPropertySchema,
    },
    Csv {
        common_property: CompiledCommonPropertySchema,
        property: csv::CsvPropertySchema,
    },
    Tsv {
        common_property: CompiledCommonPropertySchema,
        property: csv::CsvPropertySchema,
    },
}

#[derive(Debug, Clone)]
struct CompiledCommonPropertySchema {
    expr: rhai::AST,
}

impl Processor for FeatureReader {
    fn initialize(&mut self, _ctx: NodeContext) {}

    fn num_threads(&self) -> usize {
        10
    }

    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        match self {
            FeatureReader {
                params: CompiledFeatureReaderParam::CityGML { common_property },
            } => citygml::read_citygml(common_property, ctx, fw).map_err(|e| e.into()),
            FeatureReader {
                params:
                    CompiledFeatureReaderParam::Csv {
                        common_property,
                        property,
                    },
            } => csv::read_csv(
                reearth_flow_common::csv::Delimiter::Comma,
                common_property,
                property,
                ctx,
                fw,
            )
            .map_err(|e| e.into()),
            FeatureReader {
                params:
                    CompiledFeatureReaderParam::Tsv {
                        common_property,
                        property,
                    },
            } => csv::read_csv(
                reearth_flow_common::csv::Delimiter::Tab,
                common_property,
                property,
                ctx,
                fw,
            )
            .map_err(|e| e.into()),
        }
    }

    fn finish(
        &self,
        _ctx: NodeContext,
        _fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "FeatureReader"
    }
}
