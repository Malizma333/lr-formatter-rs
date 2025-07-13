use crate::track::{
    CameraZoomEvent, GroupBuilderBase, LineHitTrigger,
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

pub type LegacyCameraZoomTrigger = TriggeredEvent<CameraZoomEvent, LineHitTrigger>;
pub type LegacyCameraZoomTriggerBuilder = TriggeredEventBuilder<CameraZoomEvent, LineHitTrigger>;
pub type LegacyCameraZoomTriggerBuilderError = TriggeredEventBuilderError;

define_group_builder! (
    enum LegacyCameraZoomFeature { }

    struct LegacyCameraZoomGroup {
        triggers: Vec<LegacyCameraZoomTrigger>, Vec<LegacyCameraZoomTriggerBuilder>, LegacyCameraZoomTriggerBuilderError,
    }
);

impl GroupBuilder for LegacyCameraZoomGroupBuilder {
    fn build(&mut self) -> Result<Self::Output, GroupBuilderError<Self::Feature, Self::SubError>> {
        let mut triggers: Vec<LegacyCameraZoomTrigger> = vec![];

        for trigger_builder in &self.triggers {
            let trigger = trigger_builder.build().map_group_err()?;
            triggers.push(trigger);
        }

        Ok(LegacyCameraZoomGroup {
            features: self.features.clone(),
            triggers,
        })
    }
}

impl LegacyCameraZoomGroupBuilder {
    pub fn add_trigger(&mut self) -> &mut LegacyCameraZoomTriggerBuilder {
        self.triggers
            .push(LegacyCameraZoomTriggerBuilder::default().to_owned());
        self.triggers.last_mut().unwrap()
    }

    pub fn get_triggers(&mut self) -> impl Iterator<Item = &mut LegacyCameraZoomTriggerBuilder> {
        self.triggers.iter_mut()
    }
}
