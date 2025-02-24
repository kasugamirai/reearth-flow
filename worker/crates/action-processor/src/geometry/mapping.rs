use std::collections::HashMap;

use once_cell::sync::Lazy;
use reearth_flow_runtime::node::{NodeKind, ProcessorFactory};

use super::{
    area_on_area_overlayer::AreaOnAreaOverlayerFactory, bounds_extractor::BoundsExtractorFactory,
    bufferer::BuffererFactory, center_point_replacer::CenterPointReplacerFactory,
    clipper::ClipperFactory, closed_curve_filter::ClosedCurveFilterFactory,
    coercer::GeometryCoercerFactory, coordinate_system_setter::CoordinateSystemSetterFactory,
    extractor::GeometryExtractorFactory, extruder::ExtruderFactory, filter::GeometryFilterFactory,
    hole_counter::HoleCounterFactory, hole_extractor::HoleExtractorFactory,
    line_on_line_overlayer::LineOnLineOverlayerFactory,
    orientation_extractor::OrientationExtractorFactory, planarity_filter::PlanarityFilterFactory,
    refiner::RefinerFactory, replacer::GeometryReplacerFactory, reprojector::ReprojectorFactory,
    splitter::GeometrySplitterFactory,
    three_dimention_box_replacer::ThreeDimentionBoxReplacerFactory,
    three_dimention_rotator::ThreeDimentionRotatorFactory,
    two_dimention_forcer::TwoDimentionForcerFactory, validator::GeometryValidatorFactory,
    value_filter::GeometryValueFilterFactory, vertex_remover::VertexRemoverFactory,
};

pub static ACTION_MAPPINGS: Lazy<HashMap<String, NodeKind>> = Lazy::new(|| {
    let factories: Vec<Box<dyn ProcessorFactory>> = vec![
        Box::<CoordinateSystemSetterFactory>::default(),
        Box::<ExtruderFactory>::default(),
        Box::<ThreeDimentionBoxReplacerFactory>::default(),
        Box::<GeometryFilterFactory>::default(),
        Box::<GeometrySplitterFactory>::default(),
        Box::<GeometryCoercerFactory>::default(),
        Box::<ReprojectorFactory>::default(),
        Box::<TwoDimentionForcerFactory>::default(),
        Box::<GeometryExtractorFactory>::default(),
        Box::<OrientationExtractorFactory>::default(),
        Box::<GeometryFilterFactory>::default(),
        Box::<GeometryValidatorFactory>::default(),
        Box::<HoleCounterFactory>::default(),
        Box::<HoleExtractorFactory>::default(),
        Box::<PlanarityFilterFactory>::default(),
        Box::<LineOnLineOverlayerFactory>::default(),
        Box::<BuffererFactory>::default(),
        Box::<AreaOnAreaOverlayerFactory>::default(),
        Box::<GeometryReplacerFactory>::default(),
        Box::<ClosedCurveFilterFactory>::default(),
        Box::<VertexRemoverFactory>::default(),
        Box::<CenterPointReplacerFactory>::default(),
        Box::<ThreeDimentionRotatorFactory>::default(),
        Box::<BoundsExtractorFactory>::default(),
        Box::<ClipperFactory>::default(),
        Box::<RefinerFactory>::default(),
        Box::<GeometryValueFilterFactory>::default(),
    ];
    factories
        .into_iter()
        .map(|f| (f.name().to_string(), NodeKind::Processor(f)))
        .collect::<HashMap<_, _>>()
});
