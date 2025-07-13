use crate::track::{
    CameraZoomEvent, FrameBoundsTrigger, GroupBuilderBase,
    group_builder::{
        group_builder_base::GroupBuilder,
        group_builder_error::{GroupBuilderError, IntoGroupResult},
        group_builder_macro::define_group_builder,
    },
    groups::trigger::triggered_event::{
        TriggeredEvent, TriggeredEventBuilder, TriggeredEventBuilderError,
    },
};
use std::collections::HashSet;

type CameraZoomTrigger = TriggeredEvent<CameraZoomEvent, FrameBoundsTrigger>;
type CameraZoomTriggerBuilder = TriggeredEventBuilder<CameraZoomEvent, FrameBoundsTrigger>;
type CameraZoomTriggerBuilderError = TriggeredEventBuilderError;

define_group_builder! (
    enum CameraZoomFeature { }

    struct CameraZoomGroup {
        triggers: Vec<CameraZoomTrigger>, Vec<CameraZoomTriggerBuilder>, CameraZoomTriggerBuilderError,
    }
);

impl GroupBuilder for CameraZoomGroupBuilder {
    fn build(&mut self) -> Result<Self::Output, GroupBuilderError<Self::Feature, Self::SubError>> {
        let mut triggers: Vec<CameraZoomTrigger> = vec![];

        for trigger_builder in &self.triggers {
            let trigger = trigger_builder.build().map_group_err()?;
            triggers.push(trigger);
        }

        Ok(CameraZoomGroup {
            features: self.features.clone(),
            triggers,
        })
    }
}

impl CameraZoomGroupBuilder {
    pub fn add_trigger(&mut self) -> &mut CameraZoomTriggerBuilder {
        self.triggers
            .push(CameraZoomTriggerBuilder::default().to_owned());
        self.triggers.last_mut().unwrap()
    }

    pub fn get_triggers(&mut self) -> impl Iterator<Item = &mut CameraZoomTriggerBuilder> {
        self.triggers.iter_mut()
    }
}
