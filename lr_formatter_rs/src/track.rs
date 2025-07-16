mod grid_version;
mod group_builder;
mod groups;
mod line_type;
mod primitives;

use std::collections::HashSet;

pub use grid_version::GridVersion;
pub use group_builder::group_builder_error::{GroupBuilderError, IntoGroupResult};
pub use groups::{layer, line, metadata, rider, trigger};
pub use line_type::LineType;
pub use primitives::{
    BackgroundColorEvent, CameraZoomEvent, FrameBoundsTrigger, FrameReachedTrigger, LineColorEvent,
    LineHitTrigger, RGBColor, Vec2,
};

use crate::track::{
    group_builder::{
        group_builder_base::{GroupBuilder, GroupBuilderBase},
        group_builder_macro::define_group_builder,
    },
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
    fn build_group(&mut self) -> Result<Track, TrackBuilderError> {
        let metadata = self.metadata.build().map_group_err()?;
        let line_group = self.line_group.build_group().map_group_err()?;

        Self::check_feature(
            &mut self.features,
            TrackFeature::Layers,
            &self.layer_group,
            "layer_group",
        )?;

        let layer_group = match self.layer_group.as_mut() {
            Some(layer_group_builder) => Some(layer_group_builder.build_group().map_group_err()?),
            None => None,
        };

        Self::check_feature(
            &mut self.features,
            TrackFeature::RiderProperties,
            &self.rider_group,
            "rider_group",
        )?;

        let rider_group = match self.rider_group.as_mut() {
            Some(rider_group_builder) => Some(rider_group_builder.build_group().map_group_err()?),
            None => None,
        };

        Self::check_feature(
            &mut self.features,
            TrackFeature::BackgroundColorTriggers,
            &self.background_color_group,
            "background_color_group",
        )?;

        let background_color_group = match self.background_color_group.as_mut() {
            Some(background_color_group) => {
                Some(background_color_group.build_group().map_group_err()?)
            }
            None => None,
        };

        Self::check_feature(
            &mut self.features,
            TrackFeature::LineColorTriggers,
            &self.line_color_group,
            "line_color_group",
        )?;

        let line_color_group = match self.line_color_group.as_mut() {
            Some(line_color_group) => Some(line_color_group.build_group().map_group_err()?),
            None => None,
        };

        Self::check_feature(
            &mut self.features,
            TrackFeature::CameraZoomTriggers,
            &self.camera_zoom_group,
            "camera_zoom_group",
        )?;

        let camera_zoom_group = match self.camera_zoom_group.as_mut() {
            Some(camera_zoom_group) => Some(camera_zoom_group.build_group().map_group_err()?),
            None => None,
        };

        Self::check_feature(
            &mut self.features,
            TrackFeature::LegacyCameraZoomTriggers,
            &self.legacy_camera_zoom_group,
            "legacy_camera_zoom_group",
        )?;

        let legacy_camera_zoom_group = match self.legacy_camera_zoom_group.as_mut() {
            Some(legacy_camera_zoom_group) => {
                Some(legacy_camera_zoom_group.build_group().map_group_err()?)
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

    pub fn layer_group(&mut self) -> &mut LayerGroupBuilder {
        Self::require_feature(
            &mut self.features,
            TrackFeature::Layers,
            &mut self.layer_group,
            LayerGroupBuilder::default(),
        )
    }

    pub fn rider_group(&mut self) -> &mut RiderGroupBuilder {
        Self::require_feature(
            &mut self.features,
            TrackFeature::RiderProperties,
            &mut self.rider_group,
            RiderGroupBuilder::default(),
        )
    }

    pub fn background_color_group(&mut self) -> &mut BackgroundColorGroupBuilder {
        Self::require_feature(
            &mut self.features,
            TrackFeature::BackgroundColorTriggers,
            &mut self.background_color_group,
            BackgroundColorGroupBuilder::default(),
        )
    }

    pub fn line_color_group(&mut self) -> &mut LineColorGroupBuilder {
        Self::require_feature(
            &mut self.features,
            TrackFeature::LineColorTriggers,
            &mut self.line_color_group,
            LineColorGroupBuilder::default(),
        )
    }

    pub fn camera_zoom_group(&mut self) -> &mut CameraZoomGroupBuilder {
        Self::require_feature(
            &mut self.features,
            TrackFeature::CameraZoomTriggers,
            &mut self.camera_zoom_group,
            CameraZoomGroupBuilder::default(),
        )
    }

    pub fn legacy_camera_zoom_group(&mut self) -> &mut LegacyCameraZoomGroupBuilder {
        Self::require_feature(
            &mut self.features,
            TrackFeature::LegacyCameraZoomTriggers,
            &mut self.legacy_camera_zoom_group,
            LegacyCameraZoomGroupBuilder::default(),
        )
    }

    pub fn build(&mut self) -> Result<Track, GroupBuilderError<TrackSubBuilderError>> {
        self.build_group()
    }
}
