use std::collections::HashMap;

use reearth_flow_geometry::types::{
    coordinate::Coordinate, geometry::Geometry3D as FlowGeometry3D, rect::Rect,
};
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::{Attribute, AttributeValue, Geometry, GeometryValue};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::GeometryProcessorError;

#[derive(Debug, Clone, Default)]
pub struct ThreeDimentionBoxReplacerFactory;

impl ProcessorFactory for ThreeDimentionBoxReplacerFactory {
    fn name(&self) -> &str {
        "ThreeDimentionBoxReplacer"
    }

    fn description(&self) -> &str {
        "Replaces a three dimention box with a polygon."
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(ThreeDimentionBoxReplacer))
    }

    fn categories(&self) -> &[&'static str] {
        &["Geometry"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn build(
        &self,
        _ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let processor: ThreeDimentionBoxReplacer = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                GeometryProcessorError::ThreeDimentionBoxReplacerFactory(format!(
                    "Failed to serialize with: {}",
                    e
                ))
            })?;
            serde_json::from_value(value).map_err(|e| {
                GeometryProcessorError::ThreeDimentionBoxReplacerFactory(format!(
                    "Failed to deserialize with: {}",
                    e
                ))
            })?
        } else {
            return Err(GeometryProcessorError::ThreeDimentionBoxReplacerFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };
        Ok(Box::new(processor))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDimentionBoxReplacer {
    min_x: Attribute,
    min_y: Attribute,
    min_z: Attribute,
    max_x: Attribute,
    max_y: Attribute,
    max_z: Attribute,
}

impl Processor for ThreeDimentionBoxReplacer {
    fn initialize(&mut self, _ctx: NodeContext) {}

    fn num_threads(&self) -> usize {
        5
    }

    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        let attributes = &ctx.feature.attributes;
        let min_x = parse_f64(attributes.get(&self.min_x))?;
        let min_y = parse_f64(attributes.get(&self.min_y))?;
        let min_z = parse_f64(attributes.get(&self.min_z))?;
        let max_x = parse_f64(attributes.get(&self.max_x))?;
        let max_y = parse_f64(attributes.get(&self.max_y))?;
        let max_z = parse_f64(attributes.get(&self.max_z))?;
        let min = Coordinate::new__(min_x, min_y, min_z);
        let max = Coordinate::new__(max_x, max_y, max_z);
        let rectangle = Rect::new(min, max);
        let geometry = Geometry::with_value(GeometryValue::FlowGeometry3D(
            FlowGeometry3D::Polygon(rectangle.to_polygon()),
        ));
        let mut feature = ctx.feature.clone();
        feature.geometry = Some(geometry);
        fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
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
        "ThreeDimentionBoxReplacer"
    }
}

fn parse_f64(value: Option<&AttributeValue>) -> super::errors::Result<f64> {
    if let Some(AttributeValue::Number(min_x)) = value {
        min_x
            .as_f64()
            .ok_or(GeometryProcessorError::ThreeDimentionBoxReplacer(
                "failed to parse f64".to_string(),
            ))
    } else {
        Err(GeometryProcessorError::ThreeDimentionBoxReplacer(
            "failed to parse f64".to_string(),
        ))
    }
}
