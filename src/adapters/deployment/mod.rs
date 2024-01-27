pub mod resources;

use crate::domain::error::Error;

// Example of how you would handle Error in Rust so you can have service error wrap infra-related errors.
impl From<kube::error::Error> for Error {
    fn from(value: kube::error::Error) -> Self {
        eprintln!("{}", value);
        match value {
            kube::Error::SerdeError(_) => Self::ParsingError,
            kube::Error::InferConfig(_) => Self::DeploymentServiceCreationFailed,
            _ => Self::UncaughtError,
        }
    }
}
