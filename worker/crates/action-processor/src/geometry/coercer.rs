use std::collections::HashMap;

use reearth_flow_geometry::types::geometry::Geometry2D;
use reearth_flow_geometry::types::geometry::Geometry3D;
use reearth_flow_geometry::types::multi_line_string::{MultiLineString2D, MultiLineString3D};
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::{CityGmlGeometry, Feature, Geometry, GeometryValue};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::GeometryProcessorError;

#[derive(Debug, Clone, Default)]
pub struct GeometryCoercerFactory;

impl ProcessorFactory for GeometryCoercerFactory {
    fn name(&self) -> &str {
        "GeometryCoercer"
    }

    fn description(&self) -> &str {
        "Coerces the geometry of a feature to a specific geometry"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(GeometryCoercer))
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
        let coercer: GeometryCoercer = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                GeometryProcessorError::GeometryCoercerFactory(format!(
                    "Failed to serialize 'with' parameter: {}",
                    e
                ))
            })?;
            serde_json::from_value(value).map_err(|e| {
                GeometryProcessorError::GeometryCoercerFactory(format!(
                    "Failed to deserialize 'with' parameter: {}",
                    e
                ))
            })?
        } else {
            return Err(GeometryProcessorError::GeometryCoercerFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };
        Ok(Box::new(coercer))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
enum CoercerType {
    #[serde(rename = "lineString")]
    LineString,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GeometryCoercer {
    coercer_type: CoercerType,
}

impl Processor for GeometryCoercer {
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
            GeometryValue::CityGmlGeometry(geos) => {
                self.handle_city_gml_geometry(geos, feature, geometry, &ctx, fw);
            }
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
        "GeometryCoercer"
    }
}

impl GeometryCoercer {
    fn handle_2d_geometry(
        &self,
        geos: &Geometry2D,
        feature: &Feature,
        geometry: &Geometry,
        ctx: &ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) {
        match geos {
            Geometry2D::Polygon(polygon) => {
                let mut feature = feature.clone();
                match self.coercer_type {
                    CoercerType::LineString => {
                        let line_strings = polygon.rings().to_vec();
                        let geo = GeometryValue::FlowGeometry2D(Geometry2D::MultiLineString(
                            MultiLineString2D::new(line_strings),
                        ));
                        let mut geometry = geometry.clone();
                        geometry.value = geo;
                        feature.geometry = Some(geometry);
                    }
                }
                fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
            }
            Geometry2D::MultiPolygon(polygons) => {
                let mut feature = feature.clone();
                match self.coercer_type {
                    CoercerType::LineString => {
                        let mut geometries = Vec::<Geometry2D>::new();
                        for polygon in polygons.iter() {
                            let line_strings = polygon.rings().to_vec();
                            geometries.push(Geometry2D::MultiLineString(MultiLineString2D::new(
                                line_strings,
                            )));
                        }
                        let geo = GeometryValue::FlowGeometry2D(Geometry2D::GeometryCollection(
                            geometries,
                        ));
                        let mut geometry = geometry.clone();
                        geometry.value = geo;
                        feature.geometry = Some(geometry);
                    }
                }
                fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
            }
            _ => unimplemented!(),
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
        match geos {
            Geometry3D::Polygon(polygon) => {
                let mut feature = feature.clone();
                match self.coercer_type {
                    CoercerType::LineString => {
                        let line_strings = polygon.rings().to_vec();
                        let geo = GeometryValue::FlowGeometry3D(Geometry3D::MultiLineString(
                            MultiLineString3D::new(line_strings),
                        ));
                        let mut geometry = geometry.clone();
                        geometry.value = geo;
                        feature.geometry = Some(geometry);
                    }
                }
                fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
            }
            Geometry3D::MultiPolygon(polygons) => {
                let mut feature = feature.clone();
                match self.coercer_type {
                    CoercerType::LineString => {
                        let mut geometries = Vec::<Geometry3D>::new();
                        for polygon in polygons.iter() {
                            let line_strings = polygon.rings().to_vec();
                            geometries.push(Geometry3D::MultiLineString(MultiLineString3D::new(
                                line_strings,
                            )));
                        }
                        let geo = GeometryValue::FlowGeometry3D(Geometry3D::GeometryCollection(
                            geometries,
                        ));
                        let mut geometry = geometry.clone();
                        geometry.value = geo;
                        feature.geometry = Some(geometry);
                    }
                }
                fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
            }
            _ => unimplemented!(),
        }
    }

    fn handle_city_gml_geometry(
        &self,
        geos: &CityGmlGeometry,
        feature: &Feature,
        geometry: &Geometry,
        ctx: &ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) {
        geos.features.iter().for_each(|geo_feature| {
            let mut geometries = Vec::<Geometry3D>::new();
            for polygon in geo_feature.polygons.iter() {
                let line_strings = polygon.rings().to_vec();
                geometries.push(Geometry3D::MultiLineString(MultiLineString3D::new(
                    line_strings,
                )));
            }
            if geometries.is_empty() {
                fw.send(ctx.new_with_feature_and_port(feature.clone(), DEFAULT_PORT.clone()));
                return;
            }
            let geo = if geometries.len() == 1 {
                let Some(Geometry3D::MultiLineString(line_string)) = geometries.first() else {
                    return;
                };
                GeometryValue::FlowGeometry3D(Geometry3D::MultiLineString(line_string.clone()))
            } else {
                GeometryValue::FlowGeometry3D(Geometry3D::GeometryCollection(geometries))
            };
            let mut geometry = geometry.clone();
            geometry.value = geo;
            let mut feature = feature.clone();
            feature.id = uuid::Uuid::new_v4();
            feature.geometry = Some(geometry);
            fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
        });
    }
}
