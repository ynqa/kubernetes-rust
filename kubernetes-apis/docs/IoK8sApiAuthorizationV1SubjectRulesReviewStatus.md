# IoK8sApiAuthorizationV1SubjectRulesReviewStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**evaluation_error** | **String** | EvaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete. | [optional] 
**incomplete** | **bool** | Incomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation. | 
**non_resource_rules** | [**Vec<::models::IoK8sApiAuthorizationV1NonResourceRule>**](io.k8s.api.authorization.v1.NonResourceRule.md) | NonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete. | 
**resource_rules** | [**Vec<::models::IoK8sApiAuthorizationV1ResourceRule>**](io.k8s.api.authorization.v1.ResourceRule.md) | ResourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


