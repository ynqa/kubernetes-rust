# \AuthorizationV1beta1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_authorization_v1beta1_namespaced_local_subject_access_review**](AuthorizationV1beta1Api.md#create_authorization_v1beta1_namespaced_local_subject_access_review) | **post** /apis/authorization.k8s.io/v1beta1/namespaces/{namespace}/localsubjectaccessreviews | 
[**create_authorization_v1beta1_self_subject_access_review**](AuthorizationV1beta1Api.md#create_authorization_v1beta1_self_subject_access_review) | **post** /apis/authorization.k8s.io/v1beta1/selfsubjectaccessreviews | 
[**create_authorization_v1beta1_self_subject_rules_review**](AuthorizationV1beta1Api.md#create_authorization_v1beta1_self_subject_rules_review) | **post** /apis/authorization.k8s.io/v1beta1/selfsubjectrulesreviews | 
[**create_authorization_v1beta1_subject_access_review**](AuthorizationV1beta1Api.md#create_authorization_v1beta1_subject_access_review) | **post** /apis/authorization.k8s.io/v1beta1/subjectaccessreviews | 
[**get_authorization_v1beta1_api_resources**](AuthorizationV1beta1Api.md#get_authorization_v1beta1_api_resources) | **get** /apis/authorization.k8s.io/v1beta1/ | 


# **create_authorization_v1beta1_namespaced_local_subject_access_review**
> ::models::IoK8sApiAuthorizationV1beta1LocalSubjectAccessReview create_authorization_v1beta1_namespaced_local_subject_access_review(ctx, namespace, io_k8s_api_authorization_v1beta1_local_subject_access_review, optional)


create a LocalSubjectAccessReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_api_authorization_v1beta1_local_subject_access_review** | [**IoK8sApiAuthorizationV1beta1LocalSubjectAccessReview**](IoK8sApiAuthorizationV1beta1LocalSubjectAccessReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_api_authorization_v1beta1_local_subject_access_review** | [**IoK8sApiAuthorizationV1beta1LocalSubjectAccessReview**](IoK8sApiAuthorizationV1beta1LocalSubjectAccessReview.md)|  | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 
 **include_uninitialized** | **bool**| If IncludeUninitialized is specified, the object may be returned without completing initialization. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiAuthorizationV1beta1LocalSubjectAccessReview**](io.k8s.api.authorization.v1beta1.LocalSubjectAccessReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_authorization_v1beta1_self_subject_access_review**
> ::models::IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview create_authorization_v1beta1_self_subject_access_review(ctx, io_k8s_api_authorization_v1beta1_self_subject_access_review, optional)


create a SelfSubjectAccessReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_api_authorization_v1beta1_self_subject_access_review** | [**IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview**](IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_api_authorization_v1beta1_self_subject_access_review** | [**IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview**](IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview.md)|  | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 
 **include_uninitialized** | **bool**| If IncludeUninitialized is specified, the object may be returned without completing initialization. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview**](io.k8s.api.authorization.v1beta1.SelfSubjectAccessReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_authorization_v1beta1_self_subject_rules_review**
> ::models::IoK8sApiAuthorizationV1beta1SelfSubjectRulesReview create_authorization_v1beta1_self_subject_rules_review(ctx, io_k8s_api_authorization_v1beta1_self_subject_rules_review, optional)


create a SelfSubjectRulesReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_api_authorization_v1beta1_self_subject_rules_review** | [**IoK8sApiAuthorizationV1beta1SelfSubjectRulesReview**](IoK8sApiAuthorizationV1beta1SelfSubjectRulesReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_api_authorization_v1beta1_self_subject_rules_review** | [**IoK8sApiAuthorizationV1beta1SelfSubjectRulesReview**](IoK8sApiAuthorizationV1beta1SelfSubjectRulesReview.md)|  | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 
 **include_uninitialized** | **bool**| If IncludeUninitialized is specified, the object may be returned without completing initialization. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiAuthorizationV1beta1SelfSubjectRulesReview**](io.k8s.api.authorization.v1beta1.SelfSubjectRulesReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_authorization_v1beta1_subject_access_review**
> ::models::IoK8sApiAuthorizationV1beta1SubjectAccessReview create_authorization_v1beta1_subject_access_review(ctx, io_k8s_api_authorization_v1beta1_subject_access_review, optional)


create a SubjectAccessReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_api_authorization_v1beta1_subject_access_review** | [**IoK8sApiAuthorizationV1beta1SubjectAccessReview**](IoK8sApiAuthorizationV1beta1SubjectAccessReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_api_authorization_v1beta1_subject_access_review** | [**IoK8sApiAuthorizationV1beta1SubjectAccessReview**](IoK8sApiAuthorizationV1beta1SubjectAccessReview.md)|  | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 
 **include_uninitialized** | **bool**| If IncludeUninitialized is specified, the object may be returned without completing initialization. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiAuthorizationV1beta1SubjectAccessReview**](io.k8s.api.authorization.v1beta1.SubjectAccessReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_authorization_v1beta1_api_resources**
> ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList get_authorization_v1beta1_api_resources(ctx, )


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

