use crate::track::{
    GroupBuilderBase,
    group_builder::{
        group_builder_base::GroupBuilder,
        group_builder_error::{GroupBuilderError, IntoGroupResult},
        group_builder_macro::define_group_builder,
    },
    properties::rider::rider_base::{Rider, RiderBuilder, RiderBuilderError},
};
use std::collections::HashSet;

define_group_builder! (
    enum RiderFeature {
        StartVelocity,
        StartAngle,
        Remount,
    }

    struct RiderGroup {
        riders: Vec<Rider>, Vec<RiderBuilder>, RiderBuilderError,
    }
);

impl GroupBuilder for RiderGroupBuilder {
    fn build(&mut self) -> Result<Self::Output, GroupBuilderError<Self::Feature, Self::SubError>> {
        let mut riders: Vec<Rider> = vec![];

        for rider_builder in &self.riders {
            let rider = rider_builder.build().map_group_err()?;
            self.check_feature(
                RiderFeature::StartVelocity,
                &rider.start_velocity(),
                "start_velocity",
            )?;
            self.check_feature(
                RiderFeature::StartAngle,
                &rider.start_angle(),
                "start_angle",
            )?;
            self.check_feature(RiderFeature::Remount, &rider.can_remount(), "can_remount")?;
            riders.push(rider);
        }

        Ok(RiderGroup {
            features: self.features.clone(),
            riders,
        })
    }
}

impl RiderGroupBuilder {
    pub fn add_rider(&mut self) -> &mut RiderBuilder {
        self.riders.push(RiderBuilder::default().to_owned());
        self.riders.last_mut().unwrap()
    }

    pub fn get_riders(&mut self) -> impl Iterator<Item = &mut RiderBuilder> {
        self.riders.iter_mut()
    }
}
