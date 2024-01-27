use k8s_openapi::api::{
    apps::v1::Deployment,
    core::v1::{Pod, Service},
};
use kube::{
    api::{Patch, PatchParams},
    Api, Client,
};
use serde_json::json;

use crate::domain::{error::Error, interfaces::*};

pub struct K8sResource<T>(T);
impl TPod for K8sResource<Api<Pod>> {}

impl TDeployment for K8sResource<Api<Deployment>> {
    async fn patch(&self, name: &str, detail: String) -> Result<(), Error> {
        println!(
            "patch is being applied to {} with spec: {}...",
            name, detail
        );

        let mut patch = json!({"apiVersion": "apps/v1","kind": "Deployment","spec":{}} );
        patch["spec"] = serde_json::from_str(&detail).map_err(|_| Error::ParsingError)?;
        let params = PatchParams::apply("kubectl");
        let p = Patch::Apply(json!(patch));
        let _o_patched = self.0.patch(name, &params, &p).await?;

        println!("Done!");
        Ok(())
    }
}

impl TService for K8sResource<Api<Service>> {}

pub fn get_pod_api(client: Client, ns: &str) -> K8sResource<Api<Pod>> {
    K8sResource(Api::namespaced(client, ns))
}

pub fn get_service_api(client: Client, ns: &str) -> K8sResource<Api<Service>> {
    K8sResource(Api::namespaced(client, ns))
}
pub fn get_deployment_api(client: Client, ns: &str) -> K8sResource<Api<Deployment>> {
    K8sResource(Api::namespaced(client, ns))
}
