use crate::track::{
    FrameBoundsTrigger, GroupBuilderBase, LineColorEvent,
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

type LineColorTrigger = TriggeredEvent<LineColorEvent, FrameBoundsTrigger>;
type LineColorTriggerBuilder = TriggeredEventBuilder<LineColorEvent, FrameBoundsTrigger>;
type LineColorTriggerBuilderError = TriggeredEventBuilderError;

define_group_builder! (
    enum LineColorFeature { }

    struct LineColorGroup {
        triggers: Vec<LineColorTrigger>, Vec<LineColorTriggerBuilder>, LineColorTriggerBuilderError,
    }
);

impl GroupBuilder for LineColorGroupBuilder {
    fn build(&mut self) -> Result<Self::Output, GroupBuilderError<Self::Feature, Self::SubError>> {
        let mut triggers: Vec<LineColorTrigger> = vec![];

        for trigger_builder in &self.triggers {
            let trigger = trigger_builder.build().map_group_err()?;
            triggers.push(trigger);
        }

        Ok(LineColorGroup {
            features: self.features.clone(),
            triggers,
        })
    }
}

impl LineColorGroupBuilder {
    pub fn add_trigger(&mut self) -> &mut LineColorTriggerBuilder {
        self.triggers
            .push(LineColorTriggerBuilder::default().to_owned());
        self.triggers.last_mut().unwrap()
    }

    pub fn get_triggers(&mut self) -> impl Iterator<Item = &mut LineColorTriggerBuilder> {
        self.triggers.iter_mut()
    }
}
