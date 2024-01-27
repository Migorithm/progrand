#[derive(Debug)]
pub enum Error {
    DeploymentServiceCreationFailed,
    ParsingError,
    UncaughtError,
    WrongInput,
}
