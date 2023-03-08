// TODO: probably not needed for verification; let's just panic
use crate::{
    xdr::{self, ScStatus},
    Status,
};

#[derive(Clone, Debug)] // TODO Debug?
pub struct HostError {
    pub(crate) status: Status,
}

impl<T> From<T> for HostError
where
    Status: From<T>,
{
    fn from(status: T) -> Self {
        let status: Status = status.into();
        Self {
            status,
        }
    }
}

impl TryFrom<&HostError> for ScStatus {
    type Error = xdr::Error;
    fn try_from(err: &HostError) -> Result<Self, Self::Error> {
        err.status.try_into()
    }
}
