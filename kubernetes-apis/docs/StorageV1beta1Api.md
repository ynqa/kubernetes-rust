# \StorageV1beta1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_storage_v1beta1_storage_class**](StorageV1beta1Api.md#create_storage_v1beta1_storage_class) | **post** /apis/storage.k8s.io/v1beta1/storageclasses | 
[**create_storage_v1beta1_volume_attachment**](StorageV1beta1Api.md#create_storage_v1beta1_volume_attachment) | **post** /apis/storage.k8s.io/v1beta1/volumeattachments | 
[**delete_storage_v1beta1_collection_storage_class**](StorageV1beta1Api.md#delete_storage_v1beta1_collection_storage_class) | **delete** /apis/storage.k8s.io/v1beta1/storageclasses | 
[**delete_storage_v1beta1_collection_volume_attachment**](StorageV1beta1Api.md#delete_storage_v1beta1_collection_volume_attachment) | **delete** /apis/storage.k8s.io/v1beta1/volumeattachments | 
[**delete_storage_v1beta1_storage_class**](StorageV1beta1Api.md#delete_storage_v1beta1_storage_class) | **delete** /apis/storage.k8s.io/v1beta1/storageclasses/{name} | 
[**delete_storage_v1beta1_volume_attachment**](StorageV1beta1Api.md#delete_storage_v1beta1_volume_attachment) | **delete** /apis/storage.k8s.io/v1beta1/volumeattachments/{name} | 
[**get_storage_v1beta1_api_resources**](StorageV1beta1Api.md#get_storage_v1beta1_api_resources) | **get** /apis/storage.k8s.io/v1beta1/ | 
[**list_storage_v1beta1_storage_class**](StorageV1beta1Api.md#list_storage_v1beta1_storage_class) | **get** /apis/storage.k8s.io/v1beta1/storageclasses | 
[**list_storage_v1beta1_volume_attachment**](StorageV1beta1Api.md#list_storage_v1beta1_volume_attachment) | **get** /apis/storage.k8s.io/v1beta1/volumeattachments | 
[**patch_storage_v1beta1_storage_class**](StorageV1beta1Api.md#patch_storage_v1beta1_storage_class) | **patch** /apis/storage.k8s.io/v1beta1/storageclasses/{name} | 
[**patch_storage_v1beta1_volume_attachment**](StorageV1beta1Api.md#patch_storage_v1beta1_volume_attachment) | **patch** /apis/storage.k8s.io/v1beta1/volumeattachments/{name} | 
[**read_storage_v1beta1_storage_class**](StorageV1beta1Api.md#read_storage_v1beta1_storage_class) | **get** /apis/storage.k8s.io/v1beta1/storageclasses/{name} | 
[**read_storage_v1beta1_volume_attachment**](StorageV1beta1Api.md#read_storage_v1beta1_volume_attachment) | **get** /apis/storage.k8s.io/v1beta1/volumeattachments/{name} | 
[**replace_storage_v1beta1_storage_class**](StorageV1beta1Api.md#replace_storage_v1beta1_storage_class) | **put** /apis/storage.k8s.io/v1beta1/storageclasses/{name} | 
[**replace_storage_v1beta1_volume_attachment**](StorageV1beta1Api.md#replace_storage_v1beta1_volume_attachment) | **put** /apis/storage.k8s.io/v1beta1/volumeattachments/{name} | 
[**watch_storage_v1beta1_storage_class**](StorageV1beta1Api.md#watch_storage_v1beta1_storage_class) | **get** /apis/storage.k8s.io/v1beta1/watch/storageclasses/{name} | 
[**watch_storage_v1beta1_storage_class_list**](StorageV1beta1Api.md#watch_storage_v1beta1_storage_class_list) | **get** /apis/storage.k8s.io/v1beta1/watch/storageclasses | 
[**watch_storage_v1beta1_volume_attachment**](StorageV1beta1Api.md#watch_storage_v1beta1_volume_attachment) | **get** /apis/storage.k8s.io/v1beta1/watch/volumeattachments/{name} | 
[**watch_storage_v1beta1_volume_attachment_list**](StorageV1beta1Api.md#watch_storage_v1beta1_volume_attachment_list) | **get** /apis/storage.k8s.io/v1beta1/watch/volumeattachments | 


# **create_storage_v1beta1_storage_class**
> ::models::IoK8sApiStorageV1beta1StorageClass create_storage_v1beta1_storage_class(ctx, io_k8s_api_storage_v1beta1_storage_class, optional)


create a StorageClass

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_api_storage_v1beta1_storage_class** | [**IoK8sApiStorageV1beta1StorageClass**](IoK8sApiStorageV1beta1StorageClass.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_api_storage_v1beta1_storage_class** | [**IoK8sApiStorageV1beta1StorageClass**](IoK8sApiStorageV1beta1StorageClass.md)|  | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiStorageV1beta1StorageClass**](io.k8s.api.storage.v1beta1.StorageClass.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_storage_v1beta1_volume_attachment**
> ::models::IoK8sApiStorageV1beta1VolumeAttachment create_storage_v1beta1_volume_attachment(ctx, io_k8s_api_storage_v1beta1_volume_attachment, optional)


create a VolumeAttachment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_api_storage_v1beta1_volume_attachment** | [**IoK8sApiStorageV1beta1VolumeAttachment**](IoK8sApiStorageV1beta1VolumeAttachment.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_api_storage_v1beta1_volume_attachment** | [**IoK8sApiStorageV1beta1VolumeAttachment**](IoK8sApiStorageV1beta1VolumeAttachment.md)|  | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiStorageV1beta1VolumeAttachment**](io.k8s.api.storage.v1beta1.VolumeAttachment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_storage_v1beta1_collection_storage_class**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_storage_v1beta1_collection_storage_class(ctx, optional)


delete collection of StorageClass

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **_continue** | **String**| The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **limit** | **i32**| limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_storage_v1beta1_collection_volume_attachment**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_storage_v1beta1_collection_volume_attachment(ctx, optional)


delete collection of VolumeAttachment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **_continue** | **String**| The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **limit** | **i32**| limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_storage_v1beta1_storage_class**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_storage_v1beta1_storage_class(ctx, name, optional)


delete a StorageClass

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StorageClass | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StorageClass | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_storage_v1beta1_volume_attachment**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_storage_v1beta1_volume_attachment(ctx, name, optional)


delete a VolumeAttachment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the VolumeAttachment | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the VolumeAttachment | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_storage_v1beta1_api_resources**
> ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList get_storage_v1beta1_api_resources(ctx, )


get available resources

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList**](io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_storage_v1beta1_storage_class**
> ::models::IoK8sApiStorageV1beta1StorageClassList list_storage_v1beta1_storage_class(ctx, optional)


list or watch objects of kind StorageClass

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **_continue** | **String**| The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **limit** | **i32**| limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApiStorageV1beta1StorageClassList**](io.k8s.api.storage.v1beta1.StorageClassList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_storage_v1beta1_volume_attachment**
> ::models::IoK8sApiStorageV1beta1VolumeAttachmentList list_storage_v1beta1_volume_attachment(ctx, optional)


list or watch objects of kind VolumeAttachment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **_continue** | **String**| The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **limit** | **i32**| limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApiStorageV1beta1VolumeAttachmentList**](io.k8s.api.storage.v1beta1.VolumeAttachmentList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_storage_v1beta1_storage_class**
> ::models::IoK8sApiStorageV1beta1StorageClass patch_storage_v1beta1_storage_class(ctx, name, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified StorageClass

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StorageClass | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StorageClass | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiStorageV1beta1StorageClass**](io.k8s.api.storage.v1beta1.StorageClass.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_storage_v1beta1_volume_attachment**
> ::models::IoK8sApiStorageV1beta1VolumeAttachment patch_storage_v1beta1_volume_attachment(ctx, name, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified VolumeAttachment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the VolumeAttachment | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the VolumeAttachment | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiStorageV1beta1VolumeAttachment**](io.k8s.api.storage.v1beta1.VolumeAttachment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_storage_v1beta1_storage_class**
> ::models::IoK8sApiStorageV1beta1StorageClass read_storage_v1beta1_storage_class(ctx, name, optional)


read the specified StorageClass

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StorageClass | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StorageClass | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::IoK8sApiStorageV1beta1StorageClass**](io.k8s.api.storage.v1beta1.StorageClass.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_storage_v1beta1_volume_attachment**
> ::models::IoK8sApiStorageV1beta1VolumeAttachment read_storage_v1beta1_volume_attachment(ctx, name, optional)


read the specified VolumeAttachment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the VolumeAttachment | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the VolumeAttachment | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::IoK8sApiStorageV1beta1VolumeAttachment**](io.k8s.api.storage.v1beta1.VolumeAttachment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_storage_v1beta1_storage_class**
> ::models::IoK8sApiStorageV1beta1StorageClass replace_storage_v1beta1_storage_class(ctx, name, io_k8s_api_storage_v1beta1_storage_class, optional)


replace the specified StorageClass

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StorageClass | 
  **io_k8s_api_storage_v1beta1_storage_class** | [**IoK8sApiStorageV1beta1StorageClass**](IoK8sApiStorageV1beta1StorageClass.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StorageClass | 
 **io_k8s_api_storage_v1beta1_storage_class** | [**IoK8sApiStorageV1beta1StorageClass**](IoK8sApiStorageV1beta1StorageClass.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiStorageV1beta1StorageClass**](io.k8s.api.storage.v1beta1.StorageClass.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_storage_v1beta1_volume_attachment**
> ::models::IoK8sApiStorageV1beta1VolumeAttachment replace_storage_v1beta1_volume_attachment(ctx, name, io_k8s_api_storage_v1beta1_volume_attachment, optional)


replace the specified VolumeAttachment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the VolumeAttachment | 
  **io_k8s_api_storage_v1beta1_volume_attachment** | [**IoK8sApiStorageV1beta1VolumeAttachment**](IoK8sApiStorageV1beta1VolumeAttachment.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the VolumeAttachment | 
 **io_k8s_api_storage_v1beta1_volume_attachment** | [**IoK8sApiStorageV1beta1VolumeAttachment**](IoK8sApiStorageV1beta1VolumeAttachment.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiStorageV1beta1VolumeAttachment**](io.k8s.api.storage.v1beta1.VolumeAttachment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_storage_v1beta1_storage_class**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_storage_v1beta1_storage_class(ctx, name, optional)


watch changes to an object of kind StorageClass. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StorageClass | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StorageClass | 
 **_continue** | **String**| The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **limit** | **i32**| limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_storage_v1beta1_storage_class_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_storage_v1beta1_storage_class_list(ctx, optional)


watch individual changes to a list of StorageClass. deprecated: use the 'watch' parameter with a list operation instead.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_continue** | **String**| The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **limit** | **i32**| limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_storage_v1beta1_volume_attachment**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_storage_v1beta1_volume_attachment(ctx, name, optional)


watch changes to an object of kind VolumeAttachment. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the VolumeAttachment | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the VolumeAttachment | 
 **_continue** | **String**| The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **limit** | **i32**| limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_storage_v1beta1_volume_attachment_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_storage_v1beta1_volume_attachment_list(ctx, optional)


watch individual changes to a list of VolumeAttachment. deprecated: use the 'watch' parameter with a list operation instead.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_continue** | **String**| The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **limit** | **i32**| limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

