use crate::track::{
    group_builder_error::{GroupBuilderError, IntoGroupResult},
    group_feature_access::GroupFeatureAccess,
    properties::rider::rider_base::{Rider, RiderBuilder, RiderBuilderError},
};
use derive_more::Display;
use getset::Getters;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Display, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RiderFeature {
    StartVelocity,
    StartAngle,
    Remount,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct RiderGroup {
    features: HashSet<RiderFeature>,
    riders: Vec<Rider>,
}

#[derive(Default)]
pub struct RiderGroupBuilder {
    features: HashSet<RiderFeature>,
    riders: Vec<RiderBuilder>,
}

#[derive(Debug, Error)]
pub enum RiderSubBuilderError {
    #[error("{0}")]
    Rider(#[from] RiderBuilderError),
}

pub type RiderGroupBuilderError = GroupBuilderError<RiderFeature, RiderSubBuilderError>;

impl GroupFeatureAccess<RiderFeature, RiderSubBuilderError> for RiderGroupBuilder {}

impl RiderGroupBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_feature(&mut self, feature: RiderFeature) -> &mut Self {
        self.features.insert(feature);
        self
    }

    pub fn add_rider(&mut self) -> &mut RiderBuilder {
        self.riders.push(RiderBuilder::default().to_owned());
        self.riders.last_mut().unwrap()
    }

    pub fn get_riders(&mut self) -> impl Iterator<Item = &mut RiderBuilder> {
        self.riders.iter_mut()
    }

    pub fn build(&mut self) -> Result<RiderGroup, RiderGroupBuilderError> {
        let mut riders: Vec<Rider> = vec![];

        for rider_builder in &self.riders {
            let rider = rider_builder.build().map_group_err()?;
            Self::check_feature(
                &self.features,
                RiderFeature::StartVelocity,
                &rider.start_velocity(),
                "start_velocity",
            )?;
            Self::check_feature(
                &self.features,
                RiderFeature::StartAngle,
                &rider.start_angle(),
                "start_angle",
            )?;
            Self::check_feature(
                &self.features,
                RiderFeature::Remount,
                &rider.can_remount(),
                "can_remount",
            )?;
            riders.push(rider);
        }

        Ok(RiderGroup {
            features: self.features.clone(),
            riders,
        })
    }
}
