use std::collections::HashSet;
use std::fmt::Debug;

use crate::track::group_builder::group_builder_error::{GroupBuilderError, SubBuilderError};

/// A trait for builders that use feature gating and sub-builder error propagation.
pub trait GroupBuilderBase: Default {
    type Output;
    type Feature: Debug + Eq + std::hash::Hash + Copy;
    type SubError: SubBuilderError;

    /// Immutable access to the internal feature set
    fn feature_set(&self) -> &HashSet<Self::Feature>;

    /// Mutable access to the internal feature set
    fn feature_set_mut(&mut self) -> &mut HashSet<Self::Feature>;

    /// Require that a field is initialized if its feature is enabled.
    fn require_feature<'a, T>(
        features: &'a std::collections::HashSet<Self::Feature>,
        feature: Self::Feature,
        field: &'a mut Option<T>,
    ) -> Result<&'a mut T, GroupBuilderError<Self::Feature, Self::SubError>> {
        if !features.contains(&feature) {
            return Err(GroupBuilderError::MissingFeatureFlag(feature));
        }

        match field.as_mut() {
            Some(val) => Ok(val),
            None => unreachable!(
                "BUG: Feature data should have been initialized for {:?}",
                feature
            ),
        }
    }

    /// Check that a field is present *iff* the feature is enabled
    fn check_feature<T>(
        &self,
        feature: Self::Feature,
        field: &Option<T>,
        attr_name: &'static str,
    ) -> Result<(), GroupBuilderError<Self::Feature, Self::SubError>> {
        let features = self.feature_set();

        if features.contains(&feature) && field.is_none() {
            return Err(GroupBuilderError::MissingAttribute(attr_name));
        }

        if !features.contains(&feature) && field.is_some() {
            return Err(GroupBuilderError::MissingFeatureFlag(feature));
        }

        Ok(())
    }
}

pub trait GroupBuilder: GroupBuilderBase {
    /// Hook to initialize other fields when a feature is enabled
    fn on_enable_feature(&mut self, _feature: Self::Feature) {}

    fn enable_feature(&mut self, feature: Self::Feature) -> &mut Self {
        self.on_enable_feature(feature);
        self.feature_set_mut().insert(feature);
        self
    }

    /// Final build method to construct the group
    fn build(&mut self) -> Result<Self::Output, GroupBuilderError<Self::Feature, Self::SubError>>;
}
