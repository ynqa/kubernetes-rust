# IoK8sApiExtensionsV1beta1IpBlock

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cidr** | **String** | CIDR is a string representing the IP Block Valid examples are \"192.168.1.1/24\" | 
**except** | **Vec<String>** | Except is a slice of CIDRs that should not be included within an IP Block Valid examples are \"192.168.1.1/24\" Except values will be rejected if they are outside the CIDR range | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


