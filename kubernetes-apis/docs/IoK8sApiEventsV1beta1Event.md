# IoK8sApiEventsV1beta1Event

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | What action was taken/failed regarding to the regarding object. | [optional] 
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] 
**deprecated_count** | **i32** | Deprecated field assuring backward compatibility with core.v1 Event type | [optional] 
**deprecated_first_timestamp** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 
**deprecated_last_timestamp** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 
**deprecated_source** | [***::models::IoK8sApiCoreV1EventSource**](io.k8s.api.core.v1.EventSource.md) |  | [optional] 
**event_time** | **String** | MicroTime is version of Time with microsecond level precision. | 
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] 
**metadata** | [***::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.md) |  | [optional] 
**note** | **String** | Optional. A human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB. | [optional] 
**reason** | **String** | Why the action was taken. | [optional] 
**regarding** | [***::models::IoK8sApiCoreV1ObjectReference**](io.k8s.api.core.v1.ObjectReference.md) |  | [optional] 
**related** | [***::models::IoK8sApiCoreV1ObjectReference**](io.k8s.api.core.v1.ObjectReference.md) |  | [optional] 
**reporting_controller** | **String** | Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`. | [optional] 
**reporting_instance** | **String** | ID of the controller instance, e.g. `kubelet-xyzf`. | [optional] 
**series** | [***::models::IoK8sApiEventsV1beta1EventSeries**](io.k8s.api.events.v1beta1.EventSeries.md) |  | [optional] 
**_type** | **String** | Type of this event (Normal, Warning), new types could be added in the future. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


