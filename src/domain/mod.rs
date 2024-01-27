// Domain layer is not aware of adapter layers, simply consuming what implements traits
// the layer specified.
// In this project, as there is no orchestration required, service layer was not introduced.

use self::{
    commands::Request,
    enums::Command,
    error::Error,
    interfaces::{TDeployment, TPod, TService},
};

pub mod commands;
pub mod enums;
pub mod error;
pub mod interfaces;
pub struct Handler;

impl Handler {
    pub(crate) async fn run_on_deployment(
        req: Request,
        deployment_handler: impl TDeployment,
    ) -> Result<(), Error> {
        match req.command {
            Command::Patch => {
                deployment_handler
                    .patch(&req.name, req.detail.ok_or(Error::WrongInput)?)
                    .await?
            }

            // The rest of arms need to be implemented
            Command::Apply => todo!(),
            Command::Create => todo!(),
            Command::Delete => todo!(),
        }

        Ok(())
    }

    // This is not required
    pub(crate) async fn run_on_pod(req: Request, _pod_handler: impl TPod) -> Result<(), Error> {
        match req.command {
            Command::Patch => todo!(),
            Command::Apply => todo!(),
            Command::Create => todo!(),
            Command::Delete => todo!(),
        }
    }

    // This is not required
    pub(crate) async fn run_on_service(
        req: Request,
        _service_handler: impl TService,
    ) -> Result<(), Error> {
        match req.command {
            Command::Patch => todo!(),
            Command::Apply => todo!(),
            Command::Create => todo!(),
            Command::Delete => todo!(),
        }
    }
}
