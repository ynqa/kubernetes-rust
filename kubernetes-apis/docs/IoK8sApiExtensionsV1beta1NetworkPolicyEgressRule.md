# IoK8sApiExtensionsV1beta1NetworkPolicyEgressRule

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ports** | [**Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyPort>**](io.k8s.api.extensions.v1beta1.NetworkPolicyPort.md) | List of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list. | [optional] 
**to** | [**Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyPeer>**](io.k8s.api.extensions.v1beta1.NetworkPolicyPeer.md) | List of destinations for outgoing traffic of pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all destinations (traffic not restricted by destination). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the to list. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


