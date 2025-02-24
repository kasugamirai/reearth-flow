use std::{collections::HashMap, fs, os::unix::fs::MetadataExt, path::Path};

use reearth_flow_common::fs::get_dir_size;
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT, REJECTED_PORT},
};
use reearth_flow_types::{Attribute, AttributeValue};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use super::errors::AttributeProcessorError;

#[derive(Debug, Clone, Default)]
pub struct AttributeFilePathInfoExtractorFactory;

impl ProcessorFactory for AttributeFilePathInfoExtractorFactory {
    fn name(&self) -> &str {
        "AttributeFilePathInfoExtractor"
    }

    fn description(&self) -> &str {
        "Extracts file path information from attributes"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(AttributeFilePathInfoExtractor))
    }

    fn categories(&self) -> &[&'static str] {
        &["Attribute"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone(), REJECTED_PORT.clone()]
    }

    fn build(
        &self,
        _ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let processor: AttributeFilePathInfoExtractor = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                AttributeProcessorError::FilePathInfoExtractor(format!(
                    "Failed to serialize with: {}",
                    e
                ))
            })?;
            serde_json::from_value(value).map_err(|e| {
                AttributeProcessorError::FilePathInfoExtractor(format!(
                    "Failed to deserialize with: {}",
                    e
                ))
            })?
        } else {
            return Err(AttributeProcessorError::FilePathInfoExtractor(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };
        Ok(Box::new(processor))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttributeFilePathInfoExtractor {
    attribute: Attribute,
}

impl Processor for AttributeFilePathInfoExtractor {
    fn initialize(&mut self, _ctx: NodeContext) {}
    fn num_threads(&self) -> usize {
        5
    }
    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        let feature = &ctx.feature;
        let Some(path) = feature.get(&self.attribute) else {
            fw.send(ctx.new_with_feature_and_port(feature.clone(), REJECTED_PORT.clone()));
            return Ok(());
        };
        let AttributeValue::String(path) = path else {
            fw.send(ctx.new_with_feature_and_port(feature.clone(), REJECTED_PORT.clone()));
            return Ok(());
        };
        let path = Path::new(path);
        let mut attributes = feature.attributes.clone();
        if path.exists() && !path.is_symlink() {
            let metadata = fs::metadata(path)?;
            if metadata.is_dir() {
                attributes.insert(
                    Attribute::new("fileType"),
                    AttributeValue::String("Directory".to_string()),
                );
                let size = get_dir_size(path)?;
                attributes.insert(
                    Attribute::new("fileSize"),
                    AttributeValue::Number(Number::from(size)),
                );
            } else {
                attributes.insert(
                    Attribute::new("fileType"),
                    AttributeValue::String("File".to_string()),
                );
                attributes.insert(
                    Attribute::new("fileSize"),
                    AttributeValue::Number(Number::from(metadata.len())),
                );
            }

            if let Some(atime) =
                chrono::DateTime::<chrono::Utc>::from_timestamp(metadata.atime(), 0)
            {
                attributes.insert(
                    Attribute::new("fileAtime"),
                    AttributeValue::DateTime(atime.into()),
                );
            }
            if let Some(mtime) =
                chrono::DateTime::<chrono::Utc>::from_timestamp(metadata.mtime(), 0)
            {
                attributes.insert(
                    Attribute::new("fileMtime"),
                    AttributeValue::DateTime(mtime.into()),
                );
            }
            if let Some(ctime) =
                chrono::DateTime::<chrono::Utc>::from_timestamp(metadata.ctime(), 0)
            {
                attributes.insert(
                    Attribute::new("fileCtime"),
                    AttributeValue::DateTime(ctime.into()),
                );
            }
        }
        fw.send(
            ctx.new_with_feature_and_port(
                feature.with_attributes(attributes),
                DEFAULT_PORT.clone(),
            ),
        );
        Ok(())
    }

    fn finish(
        &self,
        _ctx: NodeContext,
        _fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "AttributeFilePathInfoExtractor"
    }
}
