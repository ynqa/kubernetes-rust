# IoK8sApiExtensionsV1beta1HttpIngressPath

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backend** | [***::models::IoK8sApiExtensionsV1beta1IngressBackend**](io.k8s.api.extensions.v1beta1.IngressBackend.md) |  | 
**path** | **String** | Path is an extended POSIX regex as defined by IEEE Std 1003.1, (i.e this follows the egrep/unix syntax, not the perl syntax) matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional \"path\" part of a URL as defined by RFC 3986. Paths must begin with a '/'. If unspecified, the path defaults to a catch all sending traffic to the backend. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


