use std::error::Error;
use std::fmt::{Debug, Display};

/// Marker trait for sub-builder errors
pub trait SubBuilderError: Debug + Error + Send + Sync + 'static {}
impl<T> SubBuilderError for T where T: Debug + Error + Send + Sync + 'static {}

/// A generalized builder error type for group builders
#[derive(Debug)]
pub enum GroupBuilderError<SubError>
where
    SubError: SubBuilderError,
{
    /// Error from a sub-builder
    SubBuilderError(SubError),
}

impl<SubError> Display for GroupBuilderError<SubError>
where
    SubError: SubBuilderError,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupBuilderError::SubBuilderError(err) => write!(f, "{}", err),
        }
    }
}

impl<SubError> Error for GroupBuilderError<SubError>
where
    SubError: SubBuilderError,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GroupBuilderError::SubBuilderError(e) => Some(e),
        }
    }
}

impl<SubError> From<SubError> for GroupBuilderError<SubError>
where
    SubError: SubBuilderError,
{
    fn from(err: SubError) -> Self {
        GroupBuilderError::SubBuilderError(err)
    }
}

/// Trait to help map nested builder errors into group-level builder errors
pub trait IntoGroupResult<T, SubError>
where
    SubError: SubBuilderError + Into<GroupBuilderError<SubError>>,
{
    fn map_group_err(self) -> Result<T, GroupBuilderError<SubError>>;
}

impl<T, GroupError, SubError> IntoGroupResult<T, SubError> for Result<T, GroupError>
where
    GroupError: Error + Debug + 'static + Into<SubError>,
    SubError: SubBuilderError + Into<GroupBuilderError<SubError>>,
{
    fn map_group_err(self) -> Result<T, GroupBuilderError<SubError>> {
        self.map_err(|e| e.into().into())
    }
}
