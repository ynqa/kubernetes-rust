extern crate failure;
extern crate kubernetes;
extern crate kubernetes_apis;

use kubernetes::config;
use kubernetes_apis::apis::client::APIClient;

fn main() {
    let kubeconfig = config::load_kube_config().expect("failed to load kubeconfig");
    let kubeclient = APIClient::new(kubeconfig);
    let res = kubeclient
        .core_v1_api()
        .list_core_v1_namespaced_pod("kube-system", true, "", "", "", "", 1, "", 1, false)
        .expect("failed to list up pods");
    println!("{:?}", res);
}
