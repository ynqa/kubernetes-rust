# IoK8sApiAppsV1StatefulSetStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collision_count** | **i32** | collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision. | [optional] 
**conditions** | [**Vec<::models::IoK8sApiAppsV1StatefulSetCondition>**](io.k8s.api.apps.v1.StatefulSetCondition.md) | Represents the latest available observations of a statefulset's current state. | [optional] 
**current_replicas** | **i32** | currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision. | [optional] 
**current_revision** | **String** | currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas). | [optional] 
**observed_generation** | **i64** | observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server. | [optional] 
**ready_replicas** | **i32** | readyReplicas is the number of Pods created by the StatefulSet controller that have a Ready Condition. | [optional] 
**replicas** | **i32** | replicas is the number of Pods created by the StatefulSet controller. | 
**update_revision** | **String** | updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas) | [optional] 
**updated_replicas** | **i32** | updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


