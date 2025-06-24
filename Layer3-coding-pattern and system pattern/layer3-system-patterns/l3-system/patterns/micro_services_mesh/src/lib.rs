//! List pods in the cluster – side-car mesh could mutate here.

use anyhow::Result;
use kube::{Client, api::ListParams};

pub async fn list_pods() -> Result<Vec<String>> {
    let client = Client::try_default().await?;
    let pods = kube::Api::<k8s_openapi::api::core::v1::Pod>::all(client)
        .list(&ListParams::default())
        .await?;
    Ok(pods.iter().map(|p| p.name_any()).collect())
}

#[cfg(test)]
mod tests {
    #[tokio::test] async fn compiles() { assert!(true); } // requires KUBECONFIG at runtime
}
