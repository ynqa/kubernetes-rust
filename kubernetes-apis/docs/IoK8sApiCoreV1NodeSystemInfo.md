# IoK8sApiCoreV1NodeSystemInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architecture** | **String** | The Architecture reported by the node | 
**boot_id** | **String** | Boot ID reported by the node. | 
**container_runtime_version** | **String** | ContainerRuntime Version reported by the node through runtime remote API (e.g. docker://1.5.0). | 
**kernel_version** | **String** | Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64). | 
**kube_proxy_version** | **String** | KubeProxy Version reported by the node. | 
**kubelet_version** | **String** | Kubelet Version reported by the node. | 
**machine_id** | **String** | MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html | 
**operating_system** | **String** | The Operating System reported by the node | 
**os_image** | **String** | OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)). | 
**system_uuid** | **String** | SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-US/Red_Hat_Subscription_Management/1/html/RHSM/getting-system-uuid.html | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


