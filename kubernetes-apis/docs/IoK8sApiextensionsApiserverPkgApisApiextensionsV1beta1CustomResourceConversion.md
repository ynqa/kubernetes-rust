# IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceConversion

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strategy** | **String** | `strategy` specifies the conversion strategy. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the CR. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information is needed for this option. | 
**webhook_client_config** | [***::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1WebhookClientConfig**](io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.WebhookClientConfig.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


