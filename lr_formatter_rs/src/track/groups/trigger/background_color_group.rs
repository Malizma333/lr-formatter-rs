use crate::track::{
    BackgroundColorEvent, FrameBoundsTrigger, GroupBuilderBase,
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

pub type BackgroundColorTrigger = TriggeredEvent<BackgroundColorEvent, FrameBoundsTrigger>;
pub type BackgroundColorTriggerBuilder =
    TriggeredEventBuilder<BackgroundColorEvent, FrameBoundsTrigger>;
pub type BackgroundColorTriggerBuilderError = TriggeredEventBuilderError;

define_group_builder! (
    enum BackgroundColorFeature { }

    struct BackgroundColorGroup {
        triggers: Vec<BackgroundColorTrigger>, Vec<BackgroundColorTriggerBuilder>, BackgroundColorTriggerBuilderError,
    }
);

impl GroupBuilder for BackgroundColorGroupBuilder {
    fn build_group(&mut self) -> Result<Self::Output, GroupBuilderError<Self::SubError>> {
        let mut triggers: Vec<BackgroundColorTrigger> = vec![];

        for trigger_builder in &self.triggers {
            let trigger = trigger_builder.build().map_group_err()?;
            triggers.push(trigger);
        }

        Ok(BackgroundColorGroup {
            features: self.features.clone(),
            triggers,
        })
    }
}

impl BackgroundColorGroupBuilder {
    pub fn add_trigger(&mut self) -> &mut BackgroundColorTriggerBuilder {
        self.triggers
            .push(BackgroundColorTriggerBuilder::default().to_owned());
        self.triggers.last_mut().unwrap()
    }

    pub fn get_triggers(&mut self) -> impl Iterator<Item = &mut BackgroundColorTriggerBuilder> {
        self.triggers.iter_mut()
    }
}
