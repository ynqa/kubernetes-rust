# kubernetes-rust

[![Build Status](https://travis-ci.com/ynqa/kubernetes-rust.svg?branch=master)](https://travis-ci.com/ynqa/kubernetes-rust)
[![Client Capabilities](https://img.shields.io/badge/Kubernetes%20client-Bronze-blue.svg?style=plastic&colorB=cd7f32&colorA=306CE8)](http://bit.ly/kubernetes-client-capabilities-badge)
[![Client Support Level](https://img.shields.io/badge/kubernetes%20client-beta-green.svg?style=plastic&colorA=306CE8)](http://bit.ly/kubernetes-client-support-badge)

Rust client for [Kubernetes](http://kubernetes.io) API.

## Example

List all Pods on `kube-system`:

```rust
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
```
