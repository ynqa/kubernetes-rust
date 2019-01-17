# IoK8sApiCoreV1HttpGetAction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | **String** | Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead. | [optional] 
**http_headers** | [**Vec<::models::IoK8sApiCoreV1HttpHeader>**](io.k8s.api.core.v1.HTTPHeader.md) | Custom headers to set in the request. HTTP allows repeated headers. | [optional] 
**path** | **String** | Path to access on the HTTP server. | [optional] 
**port** | **String** | IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number. | 
**scheme** | **String** | Scheme to use for connecting to the host. Defaults to HTTP. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


