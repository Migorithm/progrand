use super::error::Error;

pub(crate) trait TDeployment {
    fn patch(
        &self,
        name: &str,
        detail: String,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

// methods for these traits are yet to be implemented as it's not required
pub(crate) trait TPod {}
pub(crate) trait TService {}
