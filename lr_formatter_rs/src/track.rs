mod grid_version;
mod group_builder;
mod groups;
mod line_type;
mod primitives;

use std::collections::HashSet;

pub use grid_version::GridVersion;
pub use group_builder::{
    group_builder_base::{GroupBuilder, GroupBuilderBase},
    group_builder_error::{GroupBuilderError, IntoGroupResult},
};
pub use groups::{layer, line, metadata, rider, trigger};
pub use line_type::LineType;
pub use primitives::{
    BackgroundColorEvent, CameraZoomEvent, FrameBoundsTrigger, FrameReachedTrigger, LineColorEvent,
    LineHitTrigger, RGBColor, Vec2,
};

use crate::track::{
    group_builder::group_builder_macro::define_group_builder,
    groups::trigger::{
        background_color_group::{
            BackgroundColorGroup, BackgroundColorGroupBuilder, BackgroundColorGroupBuilderError,
        },
        camera_zoom_group::{CameraZoomGroup, CameraZoomGroupBuilder, CameraZoomGroupBuilderError},
        legacy_camera_zoom_group::{
            LegacyCameraZoomGroup, LegacyCameraZoomGroupBuilder, LegacyCameraZoomGroupBuilderError,
        },
        line_color_group::{LineColorGroup, LineColorGroupBuilder, LineColorGroupBuilderError},
    },
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
        BackgroundColorTriggers,
        LineColorTriggers,
        CameraZoomTriggers,
        LegacyCameraZoomTriggers
    }

    struct Track {
        metadata: Metadata, MetadataBuilder, MetadataBuilderError,
        line_group: LineGroup, LineGroupBuilder, LineGroupBuilderError,
        layer_group: Option<LayerGroup>, Option<LayerGroupBuilder>, LayerGroupBuilderError,
        rider_group: Option<RiderGroup>, Option<RiderGroupBuilder>, RiderGroupBuilderError,
        background_color_group: Option<BackgroundColorGroup>, Option<BackgroundColorGroupBuilder>, BackgroundColorGroupBuilderError,
        line_color_group: Option<LineColorGroup>, Option<LineColorGroupBuilder>, LineColorGroupBuilderError,
        camera_zoom_group: Option<CameraZoomGroup>, Option<CameraZoomGroupBuilder>, CameraZoomGroupBuilderError,
        legacy_camera_zoom_group: Option<LegacyCameraZoomGroup>, Option<LegacyCameraZoomGroupBuilder>, LegacyCameraZoomGroupBuilderError,
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

        self.check_feature(
            TrackFeature::BackgroundColorTriggers,
            &self.background_color_group,
            "background_color_group",
        )?;
        let background_color_group = match self.background_color_group.as_mut() {
            Some(background_color_group) => Some(background_color_group.build().map_group_err()?),
            None => None,
        };

        self.check_feature(
            TrackFeature::LineColorTriggers,
            &self.line_color_group,
            "line_color_group",
        )?;
        let line_color_group = match self.line_color_group.as_mut() {
            Some(line_color_group) => Some(line_color_group.build().map_group_err()?),
            None => None,
        };

        self.check_feature(
            TrackFeature::CameraZoomTriggers,
            &self.camera_zoom_group,
            "camera_zoom_group",
        )?;
        let camera_zoom_group = match self.camera_zoom_group.as_mut() {
            Some(camera_zoom_group) => Some(camera_zoom_group.build().map_group_err()?),
            None => None,
        };

        self.check_feature(
            TrackFeature::LegacyCameraZoomTriggers,
            &self.legacy_camera_zoom_group,
            "legacy_camera_zoom_group",
        )?;
        let legacy_camera_zoom_group = match self.legacy_camera_zoom_group.as_mut() {
            Some(legacy_camera_zoom_group) => {
                Some(legacy_camera_zoom_group.build().map_group_err()?)
            }
            None => None,
        };

        Ok(Track {
            features: self.features.clone(),
            metadata,
            line_group,
            layer_group,
            rider_group,
            background_color_group,
            line_color_group,
            camera_zoom_group,
            legacy_camera_zoom_group,
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

    pub fn background_color_group(
        &mut self,
    ) -> Result<&mut BackgroundColorGroupBuilder, TrackBuilderError> {
        Ok(Self::require_feature(
            &self.features,
            TrackFeature::BackgroundColorTriggers,
            &mut self.background_color_group,
        )?)
    }

    pub fn line_color_group(&mut self) -> Result<&mut LineColorGroupBuilder, TrackBuilderError> {
        Ok(Self::require_feature(
            &self.features,
            TrackFeature::LineColorTriggers,
            &mut self.line_color_group,
        )?)
    }

    pub fn camera_zoom_group(&mut self) -> Result<&mut CameraZoomGroupBuilder, TrackBuilderError> {
        Ok(Self::require_feature(
            &self.features,
            TrackFeature::CameraZoomTriggers,
            &mut self.camera_zoom_group,
        )?)
    }

    pub fn legacy_camera_zoom_group(
        &mut self,
    ) -> Result<&mut LegacyCameraZoomGroupBuilder, TrackBuilderError> {
        Ok(Self::require_feature(
            &self.features,
            TrackFeature::LegacyCameraZoomTriggers,
            &mut self.legacy_camera_zoom_group,
        )?)
    }
}
