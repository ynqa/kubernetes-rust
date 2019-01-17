# IoK8sApiCoreV1Probe

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exec** | [***::models::IoK8sApiCoreV1ExecAction**](io.k8s.api.core.v1.ExecAction.md) |  | [optional] 
**failure_threshold** | **i32** | Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1. | [optional] 
**http_get** | [***::models::IoK8sApiCoreV1HttpGetAction**](io.k8s.api.core.v1.HTTPGetAction.md) |  | [optional] 
**initial_delay_seconds** | **i32** | Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes | [optional] 
**period_seconds** | **i32** | How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. | [optional] 
**success_threshold** | **i32** | Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1. | [optional] 
**tcp_socket** | [***::models::IoK8sApiCoreV1TcpSocketAction**](io.k8s.api.core.v1.TCPSocketAction.md) |  | [optional] 
**timeout_seconds** | **i32** | Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


