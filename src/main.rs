// This is the main entry point at which object graphs are drawn and has an access to
// all the layers including adapters and domain
// Note that domain layer is not aware of adapter layers, simply consuming what implements traits
// the layer specified.

pub mod adapters;
pub mod domain;

use adapters::deployment::resources::{get_deployment_api, get_pod_api, get_service_api};
use clap::Parser;
use domain::{commands::Request, enums::Resource, error::Error, Handler};
use kube::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    // Get user input
    let request = Request::parse();
    let k8s_client = Client::try_default().await?;

    match request.resource {
        // This match arm is of our interest!
        Resource::Deployment => {
            let api = get_deployment_api(k8s_client, &request.namespace);
            Handler::run_on_deployment(request, api).await?;
        }
        Resource::Pod => {
            let api = get_pod_api(k8s_client, &request.namespace);
            Handler::run_on_pod(request, api).await?;
        }
        Resource::Service => {
            let api = get_service_api(k8s_client, &request.namespace);
            Handler::run_on_service(request, api).await?;
        }
    }

    Ok(())
}
