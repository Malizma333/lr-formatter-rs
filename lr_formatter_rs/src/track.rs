mod grid_version;
mod group_builder;
mod line_type;
mod properties;
mod rgb_color;
mod vec2;

use std::collections::HashSet;

pub use grid_version::GridVersion;
pub use group_builder::{
    group_builder_base::{GroupBuilder, GroupBuilderBase},
    group_builder_error::{GroupBuilderError, IntoGroupResult},
};
pub use line_type::LineType;
pub use properties::{layer, line, metadata, rider};
pub use rgb_color::RGBColor;
pub use vec2::Vec2;

use crate::track::{
    group_builder::group_builder_macro::define_group_builder,
    layer::layer_group::{LayerGroup, LayerGroupBuilder, LayerGroupBuilderError},
    line::line_group::{LineGroup, LineGroupBuilder, LineGroupBuilderError},
    metadata::{Metadata, MetadataBuilder, MetadataBuilderError},
    rider::rider_group::{RiderGroup, RiderGroupBuilder, RiderGroupBuilderError},
};

define_group_builder!(
    enum TrackFeature {
        RiderProperties,
        Layers,
        UseLRARemount,
        UseLegacyFakie, // TODO more descriptive name
        ZeroFrictionRiders,
        ZeroVelocityStartRiders,
        RemountRiders,
    }

    struct Track {
        metadata: Metadata, MetadataBuilder, MetadataBuilderError,
        line_group: LineGroup, LineGroupBuilder, LineGroupBuilderError,
        layer_group: Option<LayerGroup>, Option<LayerGroupBuilder>, LayerGroupBuilderError,
        rider_group: Option<RiderGroup>, Option<RiderGroupBuilder>, RiderGroupBuilderError,
    }
);

impl GroupBuilder for TrackBuilder {
    fn enable_feature(&mut self, feature: TrackFeature) -> &mut Self {
        if feature == TrackFeature::Layers && self.layer_group.is_none() {
            self.layer_group = Some(LayerGroupBuilder::default());
        }

        if feature == TrackFeature::RiderProperties && self.rider_group.is_none() {
            self.rider_group = Some(RiderGroupBuilder::default());
        }

        self.features.insert(feature);
        self
    }

    fn build(&mut self) -> Result<Track, TrackBuilderError> {
        let metadata = self.metadata.build().map_group_err()?;
        let line_group = self.line_group.build().map_group_err()?;

        self.check_feature(TrackFeature::Layers, &self.layer_group, "layer_group")?;
        let layer_group = match self.layer_group.as_mut() {
            Some(layer_group_builder) => Some(layer_group_builder.build().map_group_err()?),
            None => None,
        };

        self.check_feature(TrackFeature::Layers, &self.rider_group, "rider_group")?;
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

impl TrackBuilder {
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
            TrackFeature::RiderProperties,
            &mut self.rider_group,
        )?)
    }
}
