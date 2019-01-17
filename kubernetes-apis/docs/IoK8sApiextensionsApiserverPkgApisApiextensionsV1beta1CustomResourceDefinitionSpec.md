# IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_printer_columns** | [**Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceColumnDefinition>**](io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceColumnDefinition.md) | AdditionalPrinterColumns are additional columns shown e.g. in kubectl next to the name. Defaults to a created-at column. Optional, the global columns for all versions. Top-level and per-version columns are mutually exclusive. | [optional] 
**conversion** | [***::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceConversion**](io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceConversion.md) |  | [optional] 
**group** | **String** | Group is the group this resource belongs in | 
**names** | [***::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionNames**](io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionNames.md) |  | 
**scope** | **String** | Scope indicates whether this resource is cluster or namespace scoped.  Default is namespaced | 
**subresources** | [***::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceSubresources**](io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceSubresources.md) |  | [optional] 
**validation** | [***::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceValidation**](io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceValidation.md) |  | [optional] 
**version** | **String** | Version is the version this resource belongs in Should be always first item in Versions field if provided. Optional, but at least one of Version or Versions must be set. Deprecated: Please use `Versions`. | [optional] 
**versions** | [**Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionVersion>**](io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionVersion.md) | Versions is the list of all supported versions for this resource. If Version field is provided, this field is optional. Validation: All versions must use the same validation schema for now. i.e., top level Validation field is applied to all of these versions. Order: The version name will be used to compute the order. If the version string is \"kube-like\", it will sort above non \"kube-like\" version strings, which are ordered lexicographically. \"Kube-like\" versions start with a \"v\", then are followed by a number (the major version), then optionally the string \"alpha\" or \"beta\" and another number (the minor version). These are sorted first by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


