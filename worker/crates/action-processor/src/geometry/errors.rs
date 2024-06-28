use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub(super) enum GeometryProcessorError {
    #[error("Extruder Factory error: {0}")]
    ExtruderFactory(String),
    #[error("Extruder error: {0}")]
    Extruder(String),
    #[error("ThreeDimentionBoxReplacer Factory error: {0}")]
    ThreeDimentionBoxReplacerFactory(String),
    #[error("ThreeDimentionBoxReplacer error: {0}")]
    ThreeDimentionBoxReplacer(String),
    #[error("CoordinateSystemSetter Factory error: {0}")]
    CoordinateSystemSetterFactory(String),
    #[error("CoordinateSystemSetter error: {0}")]
    CoordinateSystemSetter(String),
    #[error("GeometryFilter Factory error: {0}")]
    GeometryFilterFactory(String),
    #[error("GeometryFilter error: {0}")]
    GeometryFilter(String),
    #[error("Reprojector Factory error: {0}")]
    ReprojectorFactory(String),
    #[error("Reprojector error: {0}")]
    Reprojector(String),
    #[error("TwoDimentionForcer Factory error: {0}")]
    TwoDimentionForcerFactory(String),
    #[error("TwoDimentionForcer error: {0}")]
    TwoDimentionForcer(String),
    #[error("GeometryExtractor Factory error: {0}")]
    GeometryExtractorFactory(String),
    #[error("GeometryExtractor error: {0}")]
    GeometryExtractor(String),
    #[error("OrientationExtractor Factory error: {0}")]
    OrientationExtractorFactory(String),
    #[error("OrientationExtractor error: {0}")]
    OrientationExtractor(String),
    #[error("GeometryValidator Factory error: {0}")]
    GeometryValidatorFactory(String),
    #[error("GeometryValidator error: {0}")]
    GeometryValidator(String),
    #[error("HoleCounter Factory error: {0}")]
    HoleCounterFactory(String),
    #[error("HoleCounter error: {0}")]
    HoleCounter(String),
    #[error("GeometryCoercer Factory error: {0}")]
    GeometryCoercerFactory(String),
    #[error("GeometryCoercer error: {0}")]
    GeometryCoercer(String),
    #[error("LineOnLineOverlayer Factory error: {0}")]
    LineOnLineOverlayerFactory(String),
    #[error("LineOnLineOverlayer error: {0}")]
    LineOnLineOverlayer(String),
}

pub(super) type Result<T, E = GeometryProcessorError> = std::result::Result<T, E>;
