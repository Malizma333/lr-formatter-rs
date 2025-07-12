use std::error::Error;
use std::fmt::{Debug, Display};

/// Marker trait for sub-builder errors
pub trait SubBuilderError: Debug + Error + Send + Sync + 'static {}
impl<T> SubBuilderError for T where T: Debug + Error + Send + Sync + 'static {}

/// A generalized builder error type for group builders
#[derive(Debug)]
pub enum GroupBuilderError<Feature, SubError>
where
    Feature: Debug,
    SubError: SubBuilderError,
{
    /// Feature flag was required (because attribute is set) but missing
    MissingFeatureFlag(Feature),

    /// Attribute was required (because feature is enabled) but missing
    MissingAttribute(&'static str),

    /// Error from a sub-builder
    SubBuilderError(SubError),
}

impl<Feature, SubError> Display for GroupBuilderError<Feature, SubError>
where
    Feature: Debug,
    SubError: SubBuilderError,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupBuilderError::MissingFeatureFlag(flag) => {
                write!(f, "Expected feature to be registered: {:?}", flag)
            }
            GroupBuilderError::MissingAttribute(attr) => {
                write!(f, "Expected attribute to be set: {}", attr)
            }
            GroupBuilderError::SubBuilderError(err) => write!(f, "{}", err),
        }
    }
}

impl<Feature, SubError> Error for GroupBuilderError<Feature, SubError>
where
    Feature: Debug,
    SubError: SubBuilderError,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GroupBuilderError::SubBuilderError(e) => Some(e),
            _ => None,
        }
    }
}

impl<Feature, SubError> From<SubError> for GroupBuilderError<Feature, SubError>
where
    Feature: Debug,
    SubError: SubBuilderError,
{
    fn from(err: SubError) -> Self {
        GroupBuilderError::SubBuilderError(err)
    }
}

/// Trait to help map nested builder errors into group-level builder errors
pub trait IntoGroupResult<T, SubError, Feature>
where
    Feature: Debug,
    SubError: SubBuilderError + Into<GroupBuilderError<Feature, SubError>>,
{
    fn map_group_err(self) -> Result<T, GroupBuilderError<Feature, SubError>>;
}

impl<T, GroupError, SubError, Feature> IntoGroupResult<T, SubError, Feature>
    for Result<T, GroupError>
where
    Feature: Debug,
    GroupError: Error + Debug + 'static + Into<SubError>,
    SubError: SubBuilderError + Into<GroupBuilderError<Feature, SubError>>,
{
    fn map_group_err(self) -> Result<T, GroupBuilderError<Feature, SubError>> {
        self.map_err(|e| e.into().into())
    }
}
