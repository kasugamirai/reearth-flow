use std::collections::HashMap;

use reearth_flow_geometry::algorithm::bufferable::Bufferable;
use reearth_flow_geometry::types::geometry::Geometry2D;
use reearth_flow_geometry::types::geometry::Geometry3D;
use reearth_flow_geometry::types::line_string::LineString2D;
use reearth_flow_runtime::node::REJECTED_PORT;
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::{Feature, Geometry, GeometryValue};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::GeometryProcessorError;

#[derive(Debug, Clone, Default)]
pub struct BuffererFactory;

impl ProcessorFactory for BuffererFactory {
    fn name(&self) -> &str {
        "Bufferer"
    }

    fn description(&self) -> &str {
        "Buffers a geometry"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(Bufferer))
    }

    fn categories(&self) -> &[&'static str] {
        &["Geometry"]
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
        let bufferer: Bufferer = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                GeometryProcessorError::BuffererFactory(format!(
                    "Failed to serialize 'with' parameter: {}",
                    e
                ))
            })?;
            serde_json::from_value(value).map_err(|e| {
                GeometryProcessorError::BuffererFactory(format!(
                    "Failed to deserialize 'with' parameter: {}",
                    e
                ))
            })?
        } else {
            return Err(GeometryProcessorError::BuffererFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };
        Ok(Box::new(bufferer))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
enum BufferType {
    #[serde(rename = "area2d")]
    Area2D,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Bufferer {
    buffer_type: BufferType,
    distance: f64,
}

impl Processor for Bufferer {
    fn initialize(&mut self, _ctx: NodeContext) {}

    fn num_threads(&self) -> usize {
        2
    }

    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        let feature = &ctx.feature;
        let Some(geometry) = &feature.geometry else {
            fw.send(ctx.new_with_feature_and_port(ctx.feature.clone(), DEFAULT_PORT.clone()));
            return Ok(());
        };
        match &geometry.value {
            GeometryValue::None => {
                fw.send(ctx.new_with_feature_and_port(feature.clone(), DEFAULT_PORT.clone()));
            }
            GeometryValue::FlowGeometry2D(geos) => {
                self.handle_2d_geometry(geos, feature, geometry, &ctx, fw);
            }
            GeometryValue::FlowGeometry3D(geos) => {
                self.handle_3d_geometry(geos, feature, geometry, &ctx, fw);
            }
            GeometryValue::CityGmlGeometry(_) => unimplemented!(),
        }
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
        "Bufferer"
    }
}

impl Bufferer {
    fn handle_2d_geometry(
        &self,
        geos: &Geometry2D,
        feature: &Feature,
        geometry: &Geometry,
        ctx: &ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) {
        match self.buffer_type {
            BufferType::Area2D => match geos {
                Geometry2D::LineString(line_string) => {
                    let mut feature = feature.clone();
                    let mut geometry = geometry.clone();
                    geometry.value = GeometryValue::FlowGeometry2D(Geometry2D::Polygon(
                        line_string.to_polygon(self.distance, 1),
                    ));
                    feature.geometry = Some(geometry);
                    fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
                }
                _ => {
                    fw.send(ctx.new_with_feature_and_port(feature.clone(), DEFAULT_PORT.clone()));
                }
            },
        }
    }

    fn handle_3d_geometry(
        &self,
        geos: &Geometry3D,
        feature: &Feature,
        geometry: &Geometry,
        ctx: &ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) {
        match self.buffer_type {
            BufferType::Area2D => match geos {
                Geometry3D::LineString(line_string) => {
                    let mut feature = feature.clone();
                    let mut geometry = geometry.clone();
                    let line_string: LineString2D<f64> = line_string.clone().into();
                    geometry.value = GeometryValue::FlowGeometry2D(Geometry2D::Polygon(
                        line_string.to_polygon(self.distance, 1),
                    ));
                    feature.geometry = Some(geometry);
                    fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
                }
                _ => {
                    let value: Geometry2D = geos.clone().into();
                    let mut geometry = geometry.clone();
                    geometry.value = GeometryValue::FlowGeometry2D(value);
                    let mut feature = feature.clone();
                    feature.geometry = Some(geometry);
                    fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
                }
            },
        }
    }
}
