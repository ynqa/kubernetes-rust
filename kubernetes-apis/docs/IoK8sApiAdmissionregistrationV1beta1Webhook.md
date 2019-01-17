# IoK8sApiAdmissionregistrationV1beta1Webhook

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_config** | [***::models::IoK8sApiAdmissionregistrationV1beta1WebhookClientConfig**](io.k8s.api.admissionregistration.v1beta1.WebhookClientConfig.md) |  | 
**failure_policy** | **String** | FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Ignore. | [optional] 
**name** | **String** | The name of the admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where \"imagepolicy\" is the name of the webhook, and kubernetes.io is the name of the organization. Required. | 
**namespace_selector** | [***::models::IoK8sApimachineryPkgApisMetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) |  | [optional] 
**rules** | [**Vec<::models::IoK8sApiAdmissionregistrationV1beta1RuleWithOperations>**](io.k8s.api.admissionregistration.v1beta1.RuleWithOperations.md) | Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. However, in order to prevent ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks from putting the cluster in a state which cannot be recovered from without completely disabling the plugin, ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks are never called on admission requests for ValidatingWebhookConfiguration and MutatingWebhookConfiguration objects. | [optional] 
**side_effects** | **String** | SideEffects states whether this webhookk has side effects. Acceptable values are: Unknown, None, Some, NoneOnDryRun Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission change and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some. Defaults to Unknown. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


