# IoK8sApiExtensionsV1beta1AllowedHostPath

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path_prefix** | **String** | pathPrefix is the path prefix that the host volume must match. It does not support `*`. Trailing slashes are trimmed when validating the path prefix with a host path.  Examples: `/foo` would allow `/foo`, `/foo/` and `/foo/bar` `/foo` would not allow `/food` or `/etc/foo` | [optional] 
**read_only** | **bool** | when set to true, will allow host volumes matching the pathPrefix only if all volume mounts are readOnly. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


