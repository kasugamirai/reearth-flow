use std::collections::HashMap;

use inflector::cases::camelcase::to_camel_case;
use once_cell::sync::Lazy;
use reearth_flow_geometry::types::geometry::{Geometry2D, Geometry3D};
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::{Feature, Geometry, GeometryFeatureType, GeometryValue};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::GeometryProcessorError;

pub static UNFILTERED_PORT: Lazy<Port> = Lazy::new(|| Port::new("unfiltered"));

#[derive(Debug, Clone, Default)]
pub struct GeometryFilterFactory;

impl ProcessorFactory for GeometryFilterFactory {
    fn name(&self) -> &str {
        "GeometryFilter"
    }

    fn description(&self) -> &str {
        "Filter geometry by type"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(GeometryFilterParam))
    }

    fn categories(&self) -> &[&'static str] {
        &["Geometry"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        let mut result = vec![UNFILTERED_PORT.clone()];
        result.extend(GeometryFilterParam::all_ports());
        result
    }

    fn build(
        &self,
        _ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let params: GeometryFilterParam = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                GeometryProcessorError::GeometryFilterFactory(format!(
                    "Failed to serialize with: {}",
                    e
                ))
            })?;
            serde_json::from_value(value).map_err(|e| {
                GeometryProcessorError::GeometryFilterFactory(format!(
                    "Failed to deserialize with: {}",
                    e
                ))
            })?
        } else {
            return Err(GeometryProcessorError::GeometryFilterFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };

        let process = GeometryFilter { params };
        Ok(Box::new(process))
    }
}

#[derive(Debug, Clone)]
pub struct GeometryFilter {
    params: GeometryFilterParam,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(tag = "filterType")]
pub enum GeometryFilterParam {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "multiple")]
    Multiple,
    #[serde(rename = "featureType")]
    FeatureType,
}

impl GeometryFilterParam {
    fn output_port(&self) -> Port {
        match self {
            GeometryFilterParam::None => Port::new("none"),
            GeometryFilterParam::Multiple => Port::new("contains"),
            GeometryFilterParam::FeatureType => unreachable!(),
        }
    }

    fn all_feature_type_ports() -> Vec<Port> {
        let mut result = reearth_flow_geometry::types::geometry::all_type_names()
            .iter()
            .map(|name| Port::new(to_camel_case(name)))
            .collect::<Vec<Port>>();
        result.extend(
            GeometryFeatureType::all_type_names()
                .iter()
                .map(|name| Port::new(to_camel_case(name)))
                .collect::<Vec<Port>>(),
        );
        result
    }

    fn all_ports() -> Vec<Port> {
        let mut result = vec![
            GeometryFilterParam::None.output_port(),
            GeometryFilterParam::Multiple.output_port(),
        ];
        result.extend(GeometryFilterParam::all_feature_type_ports());
        result
    }
}

impl Processor for GeometryFilter {
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
        match self.params {
            GeometryFilterParam::None => match &feature.geometry {
                None => fw.send(ctx.new_with_feature_and_port(
                    feature.clone(),
                    GeometryFilterParam::None.output_port(),
                )),
                Some(geomertry) => match geomertry.value {
                    GeometryValue::None => fw.send(ctx.new_with_feature_and_port(
                        feature.clone(),
                        GeometryFilterParam::None.output_port(),
                    )),
                    _ => fw.send(
                        ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone()),
                    ),
                },
            },
            GeometryFilterParam::Multiple => match &feature.geometry {
                None => {
                    fw.send(ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone()))
                }
                Some(geometry) => filter_multiple_geometry(&ctx, fw, feature, geometry),
            },
            GeometryFilterParam::FeatureType => match &feature.geometry {
                None => {
                    fw.send(ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone()))
                }
                Some(geometry) => filter_feature_type(&ctx, fw, feature, geometry),
            },
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
        "GeometryFilter"
    }
}

fn filter_multiple_geometry(
    ctx: &ExecutorContext,
    fw: &mut dyn ProcessorChannelForwarder,
    feature: &Feature,
    geometry: &Geometry,
) {
    match &geometry.value {
        GeometryValue::None => {
            fw.send(ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone()))
        }
        GeometryValue::FlowGeometry3D(geometry) => match geometry {
            Geometry3D::MultiPolygon(_) => fw.send(ctx.new_with_feature_and_port(
                feature.clone(),
                GeometryFilterParam::Multiple.output_port(),
            )),
            Geometry3D::GeometryCollection(_) => fw.send(ctx.new_with_feature_and_port(
                feature.clone(),
                GeometryFilterParam::Multiple.output_port(),
            )),
            _ => fw.send(ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone())),
        },
        GeometryValue::FlowGeometry2D(geometry) => match geometry {
            Geometry2D::MultiPolygon(_) => fw.send(ctx.new_with_feature_and_port(
                feature.clone(),
                GeometryFilterParam::Multiple.output_port(),
            )),
            Geometry2D::GeometryCollection(_) => fw.send(ctx.new_with_feature_and_port(
                feature.clone(),
                GeometryFilterParam::Multiple.output_port(),
            )),
            _ => fw.send(ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone())),
        },
        GeometryValue::CityGmlGeometry(geometry) => {
            if geometry.features.len() > 1 {
                fw.send(ctx.new_with_feature_and_port(
                    feature.clone(),
                    GeometryFilterParam::Multiple.output_port(),
                ))
            } else {
                fw.send(ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone()))
            }
        }
    }
}

fn filter_feature_type(
    ctx: &ExecutorContext,
    fw: &mut dyn ProcessorChannelForwarder,
    feature: &Feature,
    geometry: &Geometry,
) {
    match &geometry.value {
        GeometryValue::None => {
            fw.send(ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone()))
        }
        GeometryValue::FlowGeometry3D(geometry) => {
            fw.send(ctx.new_with_feature_and_port(
                feature.clone(),
                Port::new(to_camel_case(geometry.name())),
            ))
        }
        GeometryValue::FlowGeometry2D(geometry) => {
            fw.send(ctx.new_with_feature_and_port(
                feature.clone(),
                Port::new(to_camel_case(geometry.name())),
            ))
        }
        GeometryValue::CityGmlGeometry(geometry) => {
            if geometry.features.len() != 1 {
                fw.send(ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone()))
            } else {
                let Some(first_feature) = geometry.features.first() else {
                    fw.send(
                        ctx.new_with_feature_and_port(feature.clone(), UNFILTERED_PORT.clone()),
                    );
                    return;
                };
                fw.send(ctx.new_with_feature_and_port(
                    feature.clone(),
                    Port::new(to_camel_case(first_feature.name())),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::utils::{create_default_execute_context, MockProcessorChannelForwarder};

    use super::*;

    #[test]
    fn test_filter_multiple_geometry_null() {
        let mut fw = MockProcessorChannelForwarder::default();
        let feature = Feature {
            geometry: None,
            ..Default::default()
        };
        let geometry = Geometry {
            value: GeometryValue::None,
            ..Default::default()
        };
        let ctx = create_default_execute_context(&feature);
        filter_multiple_geometry(&ctx, &mut fw, &feature, &geometry);
        assert_eq!(fw.send_port, UNFILTERED_PORT.clone());
    }

    #[test]
    fn test_filter_multiple_geometry_3d_multipolygon() {
        let mut fw = MockProcessorChannelForwarder::default();
        let feature = Feature {
            geometry: Some(Geometry {
                value: GeometryValue::FlowGeometry3D(Geometry3D::MultiPolygon(Default::default())),
                ..Default::default()
            }),
            ..Default::default()
        };
        let ctx = create_default_execute_context(&feature);
        filter_multiple_geometry(&ctx, &mut fw, &feature, &feature.geometry.clone().unwrap());
        assert_eq!(fw.send_port, GeometryFilterParam::Multiple.output_port());
    }

    #[test]
    fn test_filter_multiple_geometry_3d_geometry_collection() {
        let mut fw = MockProcessorChannelForwarder::default();
        let feature = Feature {
            geometry: Some(Geometry {
                value: GeometryValue::FlowGeometry3D(Geometry3D::GeometryCollection(
                    Default::default(),
                )),
                ..Default::default()
            }),
            ..Default::default()
        };
        let ctx = create_default_execute_context(&feature);
        filter_multiple_geometry(&ctx, &mut fw, &feature, &feature.geometry.clone().unwrap());
        assert_eq!(fw.send_port, GeometryFilterParam::Multiple.output_port());
    }

    #[test]
    fn test_filter_multiple_geometry_3d_other_geometry() {
        let mut fw = MockProcessorChannelForwarder::default();
        let feature = Feature {
            geometry: Some(Geometry {
                value: GeometryValue::FlowGeometry3D(Geometry3D::Point(Default::default())),
                ..Default::default()
            }),
            ..Default::default()
        };
        let ctx = create_default_execute_context(&feature);
        filter_multiple_geometry(&ctx, &mut fw, &feature, &feature.geometry.clone().unwrap());
        assert_eq!(fw.send_port, UNFILTERED_PORT.clone());
    }

    // Add more tests for other scenarios...
}
