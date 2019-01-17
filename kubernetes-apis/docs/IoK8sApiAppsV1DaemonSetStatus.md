# IoK8sApiAppsV1DaemonSetStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collision_count** | **i32** | Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision. | [optional] 
**conditions** | [**Vec<::models::IoK8sApiAppsV1DaemonSetCondition>**](io.k8s.api.apps.v1.DaemonSetCondition.md) | Represents the latest available observations of a DaemonSet's current state. | [optional] 
**current_number_scheduled** | **i32** | The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/ | 
**desired_number_scheduled** | **i32** | The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/ | 
**number_available** | **i32** | The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds) | [optional] 
**number_misscheduled** | **i32** | The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/ | 
**number_ready** | **i32** | The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and ready. | 
**number_unavailable** | **i32** | The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds) | [optional] 
**observed_generation** | **i64** | The most recent generation observed by the daemon set controller. | [optional] 
**updated_number_scheduled** | **i32** | The total number of nodes that are running updated daemon pod | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


