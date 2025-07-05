use crate::track::{
    FeatureFieldAccess, UNREACHABLE_MESSAGE,
    trigger::{
        Event, EventBuilder, Trigger, TriggerBuilder,
        background_color::BackgroundColorEventBuilderError,
    },
};
use derive_more::Display;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Display, PartialEq, Eq, Hash)]
pub enum TriggeredEventFeature {}

pub struct TriggeredEvent<E: Event, T: Trigger> {
    trigger: T,
    event: E,
}

pub struct TriggeredEventBuilder<E: EventBuilder, T: TriggerBuilder> {
    features: HashSet<TriggeredEventFeature>,
    trigger: T,
    event: E,
}

impl<E: EventBuilder, T: TriggerBuilder> Default for TriggeredEventBuilder<E, T> {
    fn default() -> Self {
        Self {
            features: HashSet::new(),
            trigger: T::default(),
            event: E::default(),
        }
    }
}

impl<E: EventBuilder, T: TriggerBuilder>
    FeatureFieldAccess<TriggeredEventFeature, TriggeredEventBuilderError>
    for TriggeredEventBuilder<E, T>
{
    fn require_feature<'a, F>(
        current_features: &HashSet<TriggeredEventFeature>,
        field: &'a mut Option<F>,
        feature: TriggeredEventFeature,
    ) -> Result<&'a mut F, TriggeredEventBuilderError> {
        if !current_features.contains(&feature) {
            return Err(TriggeredEventBuilderError::MissingFeatureFlag(feature));
        }

        match field.as_mut() {
            Some(some_field) => Ok(some_field),
            None => unreachable!("{}", UNREACHABLE_MESSAGE),
        }
    }

    fn check_feature<F>(
        &self,
        feature: TriggeredEventFeature,
        field: &Option<F>,
        attr_name: &'static str,
    ) -> Result<(), TriggeredEventBuilderError> {
        if self.features.contains(&feature) && field.is_none() {
            return Err(TriggeredEventBuilderError::MissingAttribute(attr_name));
        }

        if !self.features.contains(&feature) && field.is_some() {
            return Err(TriggeredEventBuilderError::MissingFeatureFlag(feature));
        }

        Ok(())
    }
}

impl RiderGroupBuilder {
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
            let rider = rider_builder.build()?;
            self.check_feature(
                RiderFeature::StartAngle,
                &rider.start_angle(),
                "start_angle",
            )?;
            self.check_feature(RiderFeature::Remount, &rider.can_remount(), "can_remount")?;
            riders.push(rider);
        }

        Ok(RiderGroup { riders })
    }
}

#[derive(Error, Debug)]
pub enum TriggeredEventBuilderError {
    #[error("Expected feature to be registered before passing feature data: {0}")]
    MissingFeatureFlag(TriggeredEventFeature),
    #[error("Expected feature data to be present because feature was enabled: {0}")]
    MissingAttribute(&'static str),
    #[error("{0}")]
    BackgroundColorEventBuilderError(#[from] BackgroundColorEventBuilderError),
}
