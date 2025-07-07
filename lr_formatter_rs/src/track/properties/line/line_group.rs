use crate::track::{
    group_builder_error::{GroupBuilderError, IntoGroupResult},
    group_feature_access::GroupFeatureAccess,
    properties::line::{
        acceleration_line::{
            AccelerationLine, AccelerationLineBuilder, AccelerationLineBuilderError,
        },
        scenery_line::{SceneryLine, SceneryLineBuilder, SceneryLineBuilderError},
        standard_line::{StandardLine, StandardLineBuilder, StandardLineBuilderError},
    },
    vec2::Vec2,
};
use derive_more::Display;
use getset::Getters;
use std::{collections::HashSet, hash::Hash};
use thiserror::Error;

#[derive(Debug, Display, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LineFeature {
    SceneryWidth,
    AccelerationMultiplier,
    SinglePrecisionSceneryWidth,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct LineGroup {
    features: HashSet<LineFeature>,
    standard_lines: Vec<StandardLine>,
    acceleration_lines: Vec<AccelerationLine>,
    scenery_lines: Vec<SceneryLine>,
}

#[derive(Default)]
pub struct LineGroupBuilder {
    features: HashSet<LineFeature>,
    standard_lines: Vec<StandardLineBuilder>,
    acceleration_lines: Vec<AccelerationLineBuilder>,
    scenery_lines: Vec<SceneryLineBuilder>,
}

#[derive(Debug, Error)]
pub enum LineSubBuilderError {
    #[error("{0}")]
    StandardLine(#[from] StandardLineBuilderError),
    #[error("{0}")]
    AccelerationLine(#[from] AccelerationLineBuilderError),
    #[error("{0}")]
    SceneryLine(#[from] SceneryLineBuilderError),
}

pub type LineGroupBuilderError = GroupBuilderError<LineFeature, LineSubBuilderError>;

impl GroupFeatureAccess<LineFeature, LineSubBuilderError> for LineGroupBuilder {}

impl LineGroupBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_feature(&mut self, feature: LineFeature) -> &mut Self {
        self.features.insert(feature);
        self
    }

    pub fn add_standard_line(
        &mut self,
        id: u32,
        endpoints: (Vec2, Vec2),
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    ) -> Result<&mut StandardLineBuilder, LineGroupBuilderError> {
        self.standard_lines.push(
            StandardLineBuilder::default()
                .id(id)
                .endpoints(endpoints)
                .flipped(flipped)
                .left_extension(left_extension)
                .right_extension(right_extension)
                .to_owned(),
        );

        Ok(self.standard_lines.last_mut().unwrap())
    }

    pub fn get_standard_lines(&mut self) -> impl Iterator<Item = &mut StandardLineBuilder> {
        self.standard_lines.iter_mut()
    }

    pub fn add_acceleration_line(
        &mut self,
        id: u32,
        endpoints: (Vec2, Vec2),
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    ) -> Result<&mut AccelerationLineBuilder, LineGroupBuilderError> {
        self.acceleration_lines.push(
            AccelerationLineBuilder::default()
                .id(id)
                .endpoints(endpoints)
                .flipped(flipped)
                .left_extension(left_extension)
                .right_extension(right_extension)
                .to_owned(),
        );

        Ok(self.acceleration_lines.last_mut().unwrap())
    }

    pub fn get_acceleration_lines(&mut self) -> impl Iterator<Item = &mut AccelerationLineBuilder> {
        self.acceleration_lines.iter_mut()
    }

    pub fn add_scenery_line(
        &mut self,
        id: u32,
        endpoints: (Vec2, Vec2),
    ) -> Result<&mut SceneryLineBuilder, LineGroupBuilderError> {
        self.scenery_lines.push(
            SceneryLineBuilder::default()
                .id(id)
                .endpoints(endpoints)
                .to_owned(),
        );

        Ok(self.scenery_lines.last_mut().unwrap())
    }

    pub fn get_scenery_lines(&mut self) -> impl Iterator<Item = &mut SceneryLineBuilder> {
        self.scenery_lines.iter_mut()
    }

    pub fn build(&mut self) -> Result<LineGroup, LineGroupBuilderError> {
        let mut standard_lines: Vec<StandardLine> = vec![];
        let mut acceleration_lines: Vec<AccelerationLine> = vec![];
        let mut scenery_lines: Vec<SceneryLine> = vec![];

        for standard_line_builder in &self.standard_lines {
            let standard_line = standard_line_builder.build().map_group_err()?;
            standard_lines.push(standard_line);
        }

        for acceleration_line_builder in &self.acceleration_lines {
            let acceleration_line = acceleration_line_builder.build().map_group_err()?;
            Self::check_feature(
                &self.features,
                LineFeature::AccelerationMultiplier,
                &acceleration_line.multiplier(),
                "multiplier",
            )?;
            acceleration_lines.push(acceleration_line);
        }

        for scenery_line_builder in &self.scenery_lines {
            let scenery_line = scenery_line_builder.build().map_group_err()?;
            Self::check_feature(
                &self.features,
                LineFeature::SceneryWidth,
                &scenery_line.width(),
                "width",
            )?;
            scenery_lines.push(scenery_line);
        }

        Ok(LineGroup {
            features: self.features.clone(),
            standard_lines,
            acceleration_lines,
            scenery_lines,
        })
    }
}
