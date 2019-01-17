# \AuthenticationV1beta1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_authentication_v1beta1_token_review**](AuthenticationV1beta1Api.md#create_authentication_v1beta1_token_review) | **post** /apis/authentication.k8s.io/v1beta1/tokenreviews | 
[**get_authentication_v1beta1_api_resources**](AuthenticationV1beta1Api.md#get_authentication_v1beta1_api_resources) | **get** /apis/authentication.k8s.io/v1beta1/ | 


# **create_authentication_v1beta1_token_review**
> ::models::IoK8sApiAuthenticationV1beta1TokenReview create_authentication_v1beta1_token_review(ctx, io_k8s_api_authentication_v1beta1_token_review, optional)


create a TokenReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_api_authentication_v1beta1_token_review** | [**IoK8sApiAuthenticationV1beta1TokenReview**](IoK8sApiAuthenticationV1beta1TokenReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_api_authentication_v1beta1_token_review** | [**IoK8sApiAuthenticationV1beta1TokenReview**](IoK8sApiAuthenticationV1beta1TokenReview.md)|  | 
 **dry_run** | **String**| When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed | 
 **include_uninitialized** | **bool**| If IncludeUninitialized is specified, the object may be returned without completing initialization. | 
 **pretty** | **String**| If 'true', then the output is pretty printed. | 

### Return type

[**::models::IoK8sApiAuthenticationV1beta1TokenReview**](io.k8s.api.authentication.v1beta1.TokenReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_authentication_v1beta1_api_resources**
> ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList get_authentication_v1beta1_api_resources(ctx, )


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

