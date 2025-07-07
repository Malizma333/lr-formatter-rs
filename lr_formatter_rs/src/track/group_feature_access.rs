use crate::track::group_builder_error::GroupBuilderError;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

/// Trait to abstract feature-based field access and validation logic
pub(super) trait GroupFeatureAccess<Feature, SubError>
where
    Feature: Eq + Hash + Copy + Debug,
    SubError: std::error::Error + 'static,
{
    /// Require that an optional feature is present by its feature flag
    fn require_feature<'a, T>(
        features: &HashSet<Feature>,
        feature: Feature,
        field: &'a mut Option<T>,
    ) -> Result<&'a mut T, GroupBuilderError<Feature, SubError>> {
        if !features.contains(&feature) {
            return Err(GroupBuilderError::MissingFeatureFlag(feature));
        }

        match field.as_mut() {
            Some(val) => Ok(val),
            None => unreachable!(
                "{}",
                "BUG: Feature data should have been initialized when including feature flag"
            ),
        }
    }

    /// Validate that an optional field is present *iff* its feature is enabled.
    fn check_feature<T>(
        features: &HashSet<Feature>,
        feature: Feature,
        field: &Option<T>,
        attr_name: &'static str,
    ) -> Result<(), GroupBuilderError<Feature, SubError>> {
        if features.contains(&feature) && field.is_none() {
            return Err(GroupBuilderError::MissingAttribute(attr_name));
        }

        if !features.contains(&feature) && field.is_some() {
            return Err(GroupBuilderError::MissingFeatureFlag(feature));
        }

        Ok(())
    }
}
