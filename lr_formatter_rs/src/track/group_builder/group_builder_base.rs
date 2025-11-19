use std::fmt::Debug;

use crate::track::group_builder::group_builder_error::{GroupBuilderError, SubBuilderError};

/// A trait for builders that use feature gating and sub-builder error propagation.
pub(in crate::track) trait GroupBuilderBase: Default {
    type Feature: Debug + Eq + std::hash::Hash + Copy;
    type SubError: SubBuilderError;
    type Output;

    /// Require that a field is initialized if its feature is enabled.
    fn require_feature<'a, T>(
        features: &'a mut std::collections::HashSet<Self::Feature>,
        feature: Self::Feature,
        field: &'a mut Option<T>,
        initial: T,
    ) -> &'a mut T {
        if !features.contains(&feature) {
            features.insert(feature);
            *field = Some(initial);
        }

        match field.as_mut() {
            Some(val) => val,
            None => unreachable!(
                "BUG: Feature data should have been initialized for {:?}",
                feature
            ),
        }
    }
}

pub(in crate::track) trait GroupBuilder: GroupBuilderBase {
    /// Final build method to construct the group
    fn build_group(&mut self) -> Result<Self::Output, GroupBuilderError<Self::SubError>>;
}
