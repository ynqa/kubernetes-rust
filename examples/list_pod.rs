extern crate failure;
extern crate k8s_openapi;
extern crate kubernetes;

use k8s_openapi::api::core::v1 as api;
use k8s_openapi::List;
use kubernetes::client::APIClient;
use kubernetes::config;

#[tokio::main]
async fn main() {
    let kubeconfig = config::load_kube_config().expect("failed to load kubeconfig");
    let kubeclient = APIClient::new(kubeconfig);
    let (req, _) = api::Pod::list_namespaced_pod("kube-system", Default::default())
        .expect("failed to create a request");
    let list_pod = kubeclient
        .request::<List<api::Pod>>(req)
        .await
        .expect("failed to list up pods");
    println!("{:?}", list_pod);
}
