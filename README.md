# Deployment cli tool

# Objectives
In this project, I tried to abstract away `deployment-related concerns` so domain layer does not depend on them. 

You may be thinking that given the software engineering landscape, people would hardly ditch `Kubernetes`, but the dependency is rather a stable one, so introducing dependency inversion is close to being meaningless. 

Well, that notion is however, proven to be rather naive if history was any indication. Things change, so do will the way we handle infrastructure. 


# Design decision 
## Command Line Interface
As this is a simple command line tool for deployment, a lot of things were presumed based on the way you use Kubernetes. Specifically, you have to pass the following values: 
- namespace : namespace where the resource belongs
- resource : resource type - `deployment`, `pod`, `service` and so on.
- name : name of resource
- command : command type - `patch`, `create`, `delete`, `apply`
- detail : details to specify required changes. 

## How it works in action
```sh
# build the project
cargo build
cd target/debug
./stc-k8s --resource resource_type --name name_of_resource --namespace namespace_for_resource --command patch --detail '{"replicas":2}'
```


# Is it fully functional? 
Hands down - No. It is meant to show you how it could possibily work so you could prgrammatically operate deployment, while applying dependency injection pattern in Rust. So, only required feature, which is patch things on `Deployment` resource has been implemented. 


