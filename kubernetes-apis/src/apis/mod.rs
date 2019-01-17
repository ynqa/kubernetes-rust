use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        return Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod admissionregistration_api;
pub use self::admissionregistration_api::{ AdmissionregistrationApi, AdmissionregistrationApiClient };
mod admissionregistration_v1alpha1_api;
pub use self::admissionregistration_v1alpha1_api::{ AdmissionregistrationV1alpha1Api, AdmissionregistrationV1alpha1ApiClient };
mod admissionregistration_v1beta1_api;
pub use self::admissionregistration_v1beta1_api::{ AdmissionregistrationV1beta1Api, AdmissionregistrationV1beta1ApiClient };
mod apiextensions_api;
pub use self::apiextensions_api::{ ApiextensionsApi, ApiextensionsApiClient };
mod apiextensions_v1beta1_api;
pub use self::apiextensions_v1beta1_api::{ ApiextensionsV1beta1Api, ApiextensionsV1beta1ApiClient };
mod apiregistration_api;
pub use self::apiregistration_api::{ ApiregistrationApi, ApiregistrationApiClient };
mod apiregistration_v1_api;
pub use self::apiregistration_v1_api::{ ApiregistrationV1Api, ApiregistrationV1ApiClient };
mod apiregistration_v1beta1_api;
pub use self::apiregistration_v1beta1_api::{ ApiregistrationV1beta1Api, ApiregistrationV1beta1ApiClient };
mod apis_api;
pub use self::apis_api::{ ApisApi, ApisApiClient };
mod apps_api;
pub use self::apps_api::{ AppsApi, AppsApiClient };
mod apps_v1_api;
pub use self::apps_v1_api::{ AppsV1Api, AppsV1ApiClient };
mod apps_v1beta1_api;
pub use self::apps_v1beta1_api::{ AppsV1beta1Api, AppsV1beta1ApiClient };
mod apps_v1beta2_api;
pub use self::apps_v1beta2_api::{ AppsV1beta2Api, AppsV1beta2ApiClient };
mod auditregistration_api;
pub use self::auditregistration_api::{ AuditregistrationApi, AuditregistrationApiClient };
mod auditregistration_v1alpha1_api;
pub use self::auditregistration_v1alpha1_api::{ AuditregistrationV1alpha1Api, AuditregistrationV1alpha1ApiClient };
mod authentication_api;
pub use self::authentication_api::{ AuthenticationApi, AuthenticationApiClient };
mod authentication_v1_api;
pub use self::authentication_v1_api::{ AuthenticationV1Api, AuthenticationV1ApiClient };
mod authentication_v1beta1_api;
pub use self::authentication_v1beta1_api::{ AuthenticationV1beta1Api, AuthenticationV1beta1ApiClient };
mod authorization_api;
pub use self::authorization_api::{ AuthorizationApi, AuthorizationApiClient };
mod authorization_v1_api;
pub use self::authorization_v1_api::{ AuthorizationV1Api, AuthorizationV1ApiClient };
mod authorization_v1beta1_api;
pub use self::authorization_v1beta1_api::{ AuthorizationV1beta1Api, AuthorizationV1beta1ApiClient };
mod autoscaling_api;
pub use self::autoscaling_api::{ AutoscalingApi, AutoscalingApiClient };
mod autoscaling_v1_api;
pub use self::autoscaling_v1_api::{ AutoscalingV1Api, AutoscalingV1ApiClient };
mod autoscaling_v2beta1_api;
pub use self::autoscaling_v2beta1_api::{ AutoscalingV2beta1Api, AutoscalingV2beta1ApiClient };
mod autoscaling_v2beta2_api;
pub use self::autoscaling_v2beta2_api::{ AutoscalingV2beta2Api, AutoscalingV2beta2ApiClient };
mod batch_api;
pub use self::batch_api::{ BatchApi, BatchApiClient };
mod batch_v1_api;
pub use self::batch_v1_api::{ BatchV1Api, BatchV1ApiClient };
mod batch_v1beta1_api;
pub use self::batch_v1beta1_api::{ BatchV1beta1Api, BatchV1beta1ApiClient };
mod batch_v2alpha1_api;
pub use self::batch_v2alpha1_api::{ BatchV2alpha1Api, BatchV2alpha1ApiClient };
mod certificates_api;
pub use self::certificates_api::{ CertificatesApi, CertificatesApiClient };
mod certificates_v1beta1_api;
pub use self::certificates_v1beta1_api::{ CertificatesV1beta1Api, CertificatesV1beta1ApiClient };
mod coordination_api;
pub use self::coordination_api::{ CoordinationApi, CoordinationApiClient };
mod coordination_v1beta1_api;
pub use self::coordination_v1beta1_api::{ CoordinationV1beta1Api, CoordinationV1beta1ApiClient };
mod core_api;
pub use self::core_api::{ CoreApi, CoreApiClient };
mod core_v1_api;
pub use self::core_v1_api::{ CoreV1Api, CoreV1ApiClient };
mod events_api;
pub use self::events_api::{ EventsApi, EventsApiClient };
mod events_v1beta1_api;
pub use self::events_v1beta1_api::{ EventsV1beta1Api, EventsV1beta1ApiClient };
mod extensions_api;
pub use self::extensions_api::{ ExtensionsApi, ExtensionsApiClient };
mod extensions_v1beta1_api;
pub use self::extensions_v1beta1_api::{ ExtensionsV1beta1Api, ExtensionsV1beta1ApiClient };
mod logs_api;
pub use self::logs_api::{ LogsApi, LogsApiClient };
mod networking_api;
pub use self::networking_api::{ NetworkingApi, NetworkingApiClient };
mod networking_v1_api;
pub use self::networking_v1_api::{ NetworkingV1Api, NetworkingV1ApiClient };
mod policy_api;
pub use self::policy_api::{ PolicyApi, PolicyApiClient };
mod policy_v1beta1_api;
pub use self::policy_v1beta1_api::{ PolicyV1beta1Api, PolicyV1beta1ApiClient };
mod rbac_authorization_api;
pub use self::rbac_authorization_api::{ RbacAuthorizationApi, RbacAuthorizationApiClient };
mod rbac_authorization_v1_api;
pub use self::rbac_authorization_v1_api::{ RbacAuthorizationV1Api, RbacAuthorizationV1ApiClient };
mod rbac_authorization_v1alpha1_api;
pub use self::rbac_authorization_v1alpha1_api::{ RbacAuthorizationV1alpha1Api, RbacAuthorizationV1alpha1ApiClient };
mod rbac_authorization_v1beta1_api;
pub use self::rbac_authorization_v1beta1_api::{ RbacAuthorizationV1beta1Api, RbacAuthorizationV1beta1ApiClient };
mod scheduling_api;
pub use self::scheduling_api::{ SchedulingApi, SchedulingApiClient };
mod scheduling_v1alpha1_api;
pub use self::scheduling_v1alpha1_api::{ SchedulingV1alpha1Api, SchedulingV1alpha1ApiClient };
mod scheduling_v1beta1_api;
pub use self::scheduling_v1beta1_api::{ SchedulingV1beta1Api, SchedulingV1beta1ApiClient };
mod settings_api;
pub use self::settings_api::{ SettingsApi, SettingsApiClient };
mod settings_v1alpha1_api;
pub use self::settings_v1alpha1_api::{ SettingsV1alpha1Api, SettingsV1alpha1ApiClient };
mod storage_api;
pub use self::storage_api::{ StorageApi, StorageApiClient };
mod storage_v1_api;
pub use self::storage_v1_api::{ StorageV1Api, StorageV1ApiClient };
mod storage_v1alpha1_api;
pub use self::storage_v1alpha1_api::{ StorageV1alpha1Api, StorageV1alpha1ApiClient };
mod storage_v1beta1_api;
pub use self::storage_v1beta1_api::{ StorageV1beta1Api, StorageV1beta1ApiClient };
mod version_api;
pub use self::version_api::{ VersionApi, VersionApiClient };

pub mod configuration;
pub mod client;
