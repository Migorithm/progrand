use clap::ValueEnum;

// Enumerate k8s resource the service will manage
#[derive(Debug, ValueEnum, Clone)]
pub enum Resource {
    Pod,
    Service,
    Deployment,
}

// List of system-operatable command
#[derive(Debug, ValueEnum, Clone)]
pub enum Command {
    Patch,
    Apply,
    Create,
    Delete,
}
