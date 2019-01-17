# \RbacAuthorizationV1alpha1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#create_rbac_authorization_v1alpha1_cluster_role) | **post** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles | 
[**create_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#create_rbac_authorization_v1alpha1_cluster_role_binding) | **post** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings | 
[**create_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#create_rbac_authorization_v1alpha1_namespaced_role) | **post** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles | 
[**create_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#create_rbac_authorization_v1alpha1_namespaced_role_binding) | **post** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings | 
[**delete_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_cluster_role) | **delete** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name} | 
[**delete_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_cluster_role_binding) | **delete** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name} | 
[**delete_rbac_authorization_v1alpha1_collection_cluster_role**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_collection_cluster_role) | **delete** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles | 
[**delete_rbac_authorization_v1alpha1_collection_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_collection_cluster_role_binding) | **delete** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings | 
[**delete_rbac_authorization_v1alpha1_collection_namespaced_role**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_collection_namespaced_role) | **delete** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles | 
[**delete_rbac_authorization_v1alpha1_collection_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_collection_namespaced_role_binding) | **delete** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings | 
[**delete_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_namespaced_role) | **delete** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name} | 
[**delete_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_namespaced_role_binding) | **delete** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings/{name} | 
[**get_rbac_authorization_v1alpha1_api_resources**](RbacAuthorizationV1alpha1Api.md#get_rbac_authorization_v1alpha1_api_resources) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/ | 
[**list_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_cluster_role) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles | 
[**list_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_cluster_role_binding) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings | 
[**list_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_namespaced_role) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles | 
[**list_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_namespaced_role_binding) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings | 
[**list_rbac_authorization_v1alpha1_role_binding_for_all_namespaces**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_role_binding_for_all_namespaces) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/rolebindings | 
[**list_rbac_authorization_v1alpha1_role_for_all_namespaces**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_role_for_all_namespaces) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/roles | 
[**patch_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#patch_rbac_authorization_v1alpha1_cluster_role) | **patch** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name} | 
[**patch_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#patch_rbac_authorization_v1alpha1_cluster_role_binding) | **patch** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name} | 
[**patch_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#patch_rbac_authorization_v1alpha1_namespaced_role) | **patch** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name} | 
[**patch_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#patch_rbac_authorization_v1alpha1_namespaced_role_binding) | **patch** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings/{name} | 
[**read_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#read_rbac_authorization_v1alpha1_cluster_role) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name} | 
[**read_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#read_rbac_authorization_v1alpha1_cluster_role_binding) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name} | 
[**read_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#read_rbac_authorization_v1alpha1_namespaced_role) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name} | 
[**read_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#read_rbac_authorization_v1alpha1_namespaced_role_binding) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings/{name} | 
[**replace_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#replace_rbac_authorization_v1alpha1_cluster_role) | **put** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name} | 
[**replace_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#replace_rbac_authorization_v1alpha1_cluster_role_binding) | **put** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name} | 
[**replace_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#replace_rbac_authorization_v1alpha1_namespaced_role) | **put** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name} | 
[**replace_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#replace_rbac_authorization_v1alpha1_namespaced_role_binding) | **put** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings/{name} | 
[**watch_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_cluster_role) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterroles/{name} | 
[**watch_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_cluster_role_binding) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterrolebindings/{name} | 
[**watch_rbac_authorization_v1alpha1_cluster_role_binding_list**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_cluster_role_binding_list) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterrolebindings | 
[**watch_rbac_authorization_v1alpha1_cluster_role_list**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_cluster_role_list) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterroles | 
[**watch_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_namespaced_role) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/roles/{name} | 
[**watch_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_namespaced_role_binding) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/rolebindings/{name} | 
[**watch_rbac_authorization_v1alpha1_namespaced_role_binding_list**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_namespaced_role_binding_list) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/rolebindings | 
[**watch_rbac_authorization_v1alpha1_namespaced_role_list**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_namespaced_role_list) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/roles | 
[**watch_rbac_authorization_v1alpha1_role_binding_list_for_all_namespaces**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_role_binding_list_for_all_namespaces) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/rolebindings | 
[**watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces) | **get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/roles | 


# **create_rbac_authorization_v1alpha1_cluster_role**
> ::models::IoK8sApiRbacV1alpha1ClusterRole create_rbac_authorization_v1alpha1_cluster_role(ctx, io_k8s_api_rbac_v1alpha1_cluster_role, optional)


create a ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_api_rbac_v1alpha1_cluster_role** | [**IoK8sApiRbacV1alpha1ClusterRole**](IoK8sApiRbacV1alpha1ClusterRole.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_api_rbac_v1alpha1_cluster_role** | [**IoK8sApiRbacV1alpha1ClusterRole**](IoK8sApiRbacV1alpha1ClusterRole.md)|  | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1ClusterRole**](io.k8s.api.rbac.v1alpha1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::IoK8sApiRbacV1alpha1ClusterRoleBinding create_rbac_authorization_v1alpha1_cluster_role_binding(ctx, io_k8s_api_rbac_v1alpha1_cluster_role_binding, optional)


create a ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_api_rbac_v1alpha1_cluster_role_binding** | [**IoK8sApiRbacV1alpha1ClusterRoleBinding**](IoK8sApiRbacV1alpha1ClusterRoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_api_rbac_v1alpha1_cluster_role_binding** | [**IoK8sApiRbacV1alpha1ClusterRoleBinding**](IoK8sApiRbacV1alpha1ClusterRoleBinding.md)|  | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1ClusterRoleBinding**](io.k8s.api.rbac.v1alpha1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1alpha1_namespaced_role**
> ::models::IoK8sApiRbacV1alpha1Role create_rbac_authorization_v1alpha1_namespaced_role(ctx, namespace, io_k8s_api_rbac_v1alpha1_role, optional)


create a Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_api_rbac_v1alpha1_role** | [**IoK8sApiRbacV1alpha1Role**](IoK8sApiRbacV1alpha1Role.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_api_rbac_v1alpha1_role** | [**IoK8sApiRbacV1alpha1Role**](IoK8sApiRbacV1alpha1Role.md)|  | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1Role**](io.k8s.api.rbac.v1alpha1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::IoK8sApiRbacV1alpha1RoleBinding create_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, namespace, io_k8s_api_rbac_v1alpha1_role_binding, optional)


create a RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_api_rbac_v1alpha1_role_binding** | [**IoK8sApiRbacV1alpha1RoleBinding**](IoK8sApiRbacV1alpha1RoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_api_rbac_v1alpha1_role_binding** | [**IoK8sApiRbacV1alpha1RoleBinding**](IoK8sApiRbacV1alpha1RoleBinding.md)|  | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1RoleBinding**](io.k8s.api.rbac.v1alpha1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1alpha1_cluster_role**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1alpha1_cluster_role(ctx, name, optional)


delete a ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
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

# **delete_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, optional)


delete a ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
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

# **delete_rbac_authorization_v1alpha1_collection_cluster_role**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1alpha1_collection_cluster_role(ctx, optional)


delete collection of ClusterRole

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

# **delete_rbac_authorization_v1alpha1_collection_cluster_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1alpha1_collection_cluster_role_binding(ctx, optional)


delete collection of ClusterRoleBinding

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

# **delete_rbac_authorization_v1alpha1_collection_namespaced_role**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1alpha1_collection_namespaced_role(ctx, namespace, optional)


delete collection of Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **delete_rbac_authorization_v1alpha1_collection_namespaced_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1alpha1_collection_namespaced_role_binding(ctx, namespace, optional)


delete collection of RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **delete_rbac_authorization_v1alpha1_namespaced_role**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, optional)


delete a Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **delete_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, optional)


delete a RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **get_rbac_authorization_v1alpha1_api_resources**
> ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList get_rbac_authorization_v1alpha1_api_resources(ctx, )


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

# **list_rbac_authorization_v1alpha1_cluster_role**
> ::models::IoK8sApiRbacV1alpha1ClusterRoleList list_rbac_authorization_v1alpha1_cluster_role(ctx, optional)


list or watch objects of kind ClusterRole

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

[**::models::IoK8sApiRbacV1alpha1ClusterRoleList**](io.k8s.api.rbac.v1alpha1.ClusterRoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::IoK8sApiRbacV1alpha1ClusterRoleBindingList list_rbac_authorization_v1alpha1_cluster_role_binding(ctx, optional)


list or watch objects of kind ClusterRoleBinding

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

[**::models::IoK8sApiRbacV1alpha1ClusterRoleBindingList**](io.k8s.api.rbac.v1alpha1.ClusterRoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_namespaced_role**
> ::models::IoK8sApiRbacV1alpha1RoleList list_rbac_authorization_v1alpha1_namespaced_role(ctx, namespace, optional)


list or watch objects of kind Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

[**::models::IoK8sApiRbacV1alpha1RoleList**](io.k8s.api.rbac.v1alpha1.RoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::IoK8sApiRbacV1alpha1RoleBindingList list_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, namespace, optional)


list or watch objects of kind RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

[**::models::IoK8sApiRbacV1alpha1RoleBindingList**](io.k8s.api.rbac.v1alpha1.RoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_role_binding_for_all_namespaces**
> ::models::IoK8sApiRbacV1alpha1RoleBindingList list_rbac_authorization_v1alpha1_role_binding_for_all_namespaces(ctx, optional)


list or watch objects of kind RoleBinding

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

[**::models::IoK8sApiRbacV1alpha1RoleBindingList**](io.k8s.api.rbac.v1alpha1.RoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_role_for_all_namespaces**
> ::models::IoK8sApiRbacV1alpha1RoleList list_rbac_authorization_v1alpha1_role_for_all_namespaces(ctx, optional)


list or watch objects of kind Role

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

[**::models::IoK8sApiRbacV1alpha1RoleList**](io.k8s.api.rbac.v1alpha1.RoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1alpha1_cluster_role**
> ::models::IoK8sApiRbacV1alpha1ClusterRole patch_rbac_authorization_v1alpha1_cluster_role(ctx, name, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1ClusterRole**](io.k8s.api.rbac.v1alpha1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::IoK8sApiRbacV1alpha1ClusterRoleBinding patch_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1ClusterRoleBinding**](io.k8s.api.rbac.v1alpha1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1alpha1_namespaced_role**
> ::models::IoK8sApiRbacV1alpha1Role patch_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1Role**](io.k8s.api.rbac.v1alpha1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::IoK8sApiRbacV1alpha1RoleBinding patch_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1RoleBinding**](io.k8s.api.rbac.v1alpha1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1alpha1_cluster_role**
> ::models::IoK8sApiRbacV1alpha1ClusterRole read_rbac_authorization_v1alpha1_cluster_role(ctx, name, optional)


read the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiRbacV1alpha1ClusterRole**](io.k8s.api.rbac.v1alpha1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::IoK8sApiRbacV1alpha1ClusterRoleBinding read_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, optional)


read the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiRbacV1alpha1ClusterRoleBinding**](io.k8s.api.rbac.v1alpha1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1alpha1_namespaced_role**
> ::models::IoK8sApiRbacV1alpha1Role read_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, optional)


read the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiRbacV1alpha1Role**](io.k8s.api.rbac.v1alpha1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::IoK8sApiRbacV1alpha1RoleBinding read_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, optional)


read the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiRbacV1alpha1RoleBinding**](io.k8s.api.rbac.v1alpha1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1alpha1_cluster_role**
> ::models::IoK8sApiRbacV1alpha1ClusterRole replace_rbac_authorization_v1alpha1_cluster_role(ctx, name, io_k8s_api_rbac_v1alpha1_cluster_role, optional)


replace the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
  **io_k8s_api_rbac_v1alpha1_cluster_role** | [**IoK8sApiRbacV1alpha1ClusterRole**](IoK8sApiRbacV1alpha1ClusterRole.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **io_k8s_api_rbac_v1alpha1_cluster_role** | [**IoK8sApiRbacV1alpha1ClusterRole**](IoK8sApiRbacV1alpha1ClusterRole.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1ClusterRole**](io.k8s.api.rbac.v1alpha1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::IoK8sApiRbacV1alpha1ClusterRoleBinding replace_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, io_k8s_api_rbac_v1alpha1_cluster_role_binding, optional)


replace the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
  **io_k8s_api_rbac_v1alpha1_cluster_role_binding** | [**IoK8sApiRbacV1alpha1ClusterRoleBinding**](IoK8sApiRbacV1alpha1ClusterRoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **io_k8s_api_rbac_v1alpha1_cluster_role_binding** | [**IoK8sApiRbacV1alpha1ClusterRoleBinding**](IoK8sApiRbacV1alpha1ClusterRoleBinding.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1ClusterRoleBinding**](io.k8s.api.rbac.v1alpha1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1alpha1_namespaced_role**
> ::models::IoK8sApiRbacV1alpha1Role replace_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, io_k8s_api_rbac_v1alpha1_role, optional)


replace the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_api_rbac_v1alpha1_role** | [**IoK8sApiRbacV1alpha1Role**](IoK8sApiRbacV1alpha1Role.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_api_rbac_v1alpha1_role** | [**IoK8sApiRbacV1alpha1Role**](IoK8sApiRbacV1alpha1Role.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1Role**](io.k8s.api.rbac.v1alpha1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::IoK8sApiRbacV1alpha1RoleBinding replace_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, io_k8s_api_rbac_v1alpha1_role_binding, optional)


replace the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_api_rbac_v1alpha1_role_binding** | [**IoK8sApiRbacV1alpha1RoleBinding**](IoK8sApiRbacV1alpha1RoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_api_rbac_v1alpha1_role_binding** | [**IoK8sApiRbacV1alpha1RoleBinding**](IoK8sApiRbacV1alpha1RoleBinding.md)|  | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 

### Return type

[**::models::IoK8sApiRbacV1alpha1RoleBinding**](io.k8s.api.rbac.v1alpha1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1alpha1_cluster_role**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_cluster_role(ctx, name, optional)


watch changes to an object of kind ClusterRole. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
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

# **watch_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, optional)


watch changes to an object of kind ClusterRoleBinding. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
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

# **watch_rbac_authorization_v1alpha1_cluster_role_binding_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_cluster_role_binding_list(ctx, optional)


watch individual changes to a list of ClusterRoleBinding. deprecated: use the 'watch' parameter with a list operation instead.

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

# **watch_rbac_authorization_v1alpha1_cluster_role_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_cluster_role_list(ctx, optional)


watch individual changes to a list of ClusterRole. deprecated: use the 'watch' parameter with a list operation instead.

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

# **watch_rbac_authorization_v1alpha1_namespaced_role**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, optional)


watch changes to an object of kind Role. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **watch_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, optional)


watch changes to an object of kind RoleBinding. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **watch_rbac_authorization_v1alpha1_namespaced_role_binding_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_namespaced_role_binding_list(ctx, namespace, optional)


watch individual changes to a list of RoleBinding. deprecated: use the 'watch' parameter with a list operation instead.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **watch_rbac_authorization_v1alpha1_namespaced_role_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_namespaced_role_list(ctx, namespace, optional)


watch individual changes to a list of Role. deprecated: use the 'watch' parameter with a list operation instead.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **watch_rbac_authorization_v1alpha1_role_binding_list_for_all_namespaces**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_role_binding_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of RoleBinding. deprecated: use the 'watch' parameter with a list operation instead.

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

# **watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of Role. deprecated: use the 'watch' parameter with a list operation instead.

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

