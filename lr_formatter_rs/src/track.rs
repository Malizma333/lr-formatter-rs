mod grid_version;
mod group_builder_error;
mod group_feature_access;
mod line_type;
mod properties;
mod rgb_color;
mod vec2;

use derive_more::Display;
use getset::Getters;
use std::collections::HashSet;
use thiserror::Error;

pub use grid_version::GridVersion;
pub use line_type::LineType;
pub use properties::{layer, line, metadata, rider};
pub use rgb_color::RGBColor;
pub use vec2::Vec2;

use crate::track::{
    group_builder_error::{GroupBuilderError, IntoGroupResult},
    group_feature_access::GroupFeatureAccess,
    layer::layer_group::{LayerGroup, LayerGroupBuilder, LayerGroupBuilderError},
    line::line_group::{LineGroup, LineGroupBuilder, LineGroupBuilderError},
    metadata::{Metadata, MetadataBuilder, MetadataBuilderError},
    rider::rider_group::{RiderGroup, RiderGroupBuilder, RiderGroupBuilderError},
};

#[derive(Debug, Display, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TrackFeature {
    Riders,
    Layers,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Track {
    features: HashSet<TrackFeature>,
    metadata: Metadata,
    line_group: LineGroup,
    layer_group: Option<LayerGroup>,
    rider_group: Option<RiderGroup>,
}

#[derive(Default)]
pub struct TrackBuilder {
    features: HashSet<TrackFeature>,
    line_group: LineGroupBuilder,
    metadata: MetadataBuilder,
    layer_group: Option<LayerGroupBuilder>,
    rider_group: Option<RiderGroupBuilder>,
}

#[derive(Error, Debug)]
pub enum TrackSubBuilderError {
    #[error("{0}")]
    LineGroupBuilderError(#[from] LineGroupBuilderError),
    #[error("{0}")]
    LayerGroupBuilderError(#[from] LayerGroupBuilderError),
    #[error("{0}")]
    RiderGroupBuilderError(#[from] RiderGroupBuilderError),
    #[error("{0}")]
    MetadataBuilderError(#[from] MetadataBuilderError),
}

pub type TrackBuilderError = GroupBuilderError<TrackFeature, TrackSubBuilderError>;

impl GroupFeatureAccess<TrackFeature, TrackSubBuilderError> for TrackBuilder {}

impl TrackBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_feature(&mut self, feature: TrackFeature) -> &mut Self {
        if feature == TrackFeature::Layers && self.layer_group.is_none() {
            self.layer_group = Some(LayerGroupBuilder::default());
        }

        if feature == TrackFeature::Riders && self.rider_group.is_none() {
            self.rider_group = Some(RiderGroupBuilder::default());
        }

        self.features.insert(feature);
        self
    }

    pub fn metadata(&mut self) -> &mut MetadataBuilder {
        &mut self.metadata
    }

    pub fn line_group(&mut self) -> &mut LineGroupBuilder {
        &mut self.line_group
    }

    pub fn layer_group(&mut self) -> Result<&mut LayerGroupBuilder, TrackBuilderError> {
        Self::require_feature(&self.features, TrackFeature::Layers, &mut self.layer_group)
    }

    pub fn rider_group(&mut self) -> Result<&mut RiderGroupBuilder, TrackBuilderError> {
        Ok(Self::require_feature(
            &self.features,
            TrackFeature::Riders,
            &mut self.rider_group,
        )?)
    }

    pub fn build(&mut self) -> Result<Track, TrackBuilderError> {
        let metadata = self.metadata.build().map_group_err()?;
        let line_group = self.line_group.build().map_group_err()?;

        Self::check_feature(
            &self.features,
            TrackFeature::Layers,
            &self.layer_group,
            "layer_group",
        )?;
        let layer_group = match self.layer_group.as_mut() {
            Some(layer_group_builder) => Some(layer_group_builder.build().map_group_err()?),
            None => None,
        };

        Self::check_feature(
            &self.features,
            TrackFeature::Layers,
            &self.rider_group,
            "rider_group",
        )?;
        let rider_group = match self.rider_group.as_mut() {
            Some(rider_group_builder) => Some(rider_group_builder.build().map_group_err()?),
            None => None,
        };

        Ok(Track {
            features: self.features.clone(),
            metadata,
            line_group,
            layer_group,
            rider_group,
        })
    }
}
