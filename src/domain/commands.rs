use clap::Parser;

use super::enums::{Command, Resource};

#[derive(Parser, Debug)]
pub struct Request {
    #[clap(long, default_value = "default")]
    pub namespace: String,

    #[clap(short, long, value_parser, value_enum)]
    pub resource: Resource,

    #[clap(long)]
    pub name: String,

    #[clap(short, long, value_parser, value_enum)]
    pub command: Command,

    #[clap(short, long)]
    pub detail: Option<String>,
}
