# IoK8sApiCoordinationV1beta1LeaseSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acquire_time** | **String** | MicroTime is version of Time with microsecond level precision. | [optional] 
**holder_identity** | **String** | holderIdentity contains the identity of the holder of a current lease. | [optional] 
**lease_duration_seconds** | **i32** | leaseDurationSeconds is a duration that candidates for a lease need to wait to force acquire it. This is measure against time of last observed RenewTime. | [optional] 
**lease_transitions** | **i32** | leaseTransitions is the number of transitions of a lease between holders. | [optional] 
**renew_time** | **String** | MicroTime is version of Time with microsecond level precision. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


