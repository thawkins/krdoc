use clap::Parser;
use k8s_openapi::api::core::v1::Pod;
use std::{sync::Arc, time::Duration};
use futures::StreamExt;
use kube::{
    Api, Client, ResourceExt,
    runtime::controller::{Action, Controller}
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author="Tim Hawkins", version="0.1.0", about="program to document kubernetes clusters", long_about = None)]
struct Args {
    /// kubectl context to document
    #[arg(short, long, default_value ="")]
    context: String,
    /// kubenetes namespace to document
    #[arg(short, long, default_value ="default")]
    namespace: String,
    /// List of documentation types to produce
    #[arg(short, long, default_value ="all")]
    output: String,
    /// Wether to include all namespaces
    #[arg(short, long, default_value_t =false)]
    all_namespaces: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}
pub type Result<T, E = Error> = std::result::Result<T, E>;

fn error_policy(_object: Arc<Pod>, _err: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(5))
}

async fn reconcile(obj: Arc<Pod>, ctx: Arc<()>) -> Result<Action> {
    println!("reconcile request: {}", obj.name_any());
    Ok(Action::requeue(Duration::from_secs(3600)))
}

#[tokio::main]
async fn main() -> Result<(), kube::Error>  {
    let args = Args::parse();
    println!("context {}", args.context);
    println!("namespacet {}", args.output);
    println!("output {}", args.output);
    println!("all_namespaces {}", args.all_namespaces);

    let client = Client::try_default().await?;
    let pods = Api::<Pod>::all(client);

    Controller::new(pods.clone(), Default::default())
        .run(reconcile, error_policy, Arc::new(()))
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())




}