use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
  configuration: Rc<Configuration>,
  admissionregistration_api: Box<::apis::AdmissionregistrationApi>,
  admissionregistration_v1alpha1_api: Box<::apis::AdmissionregistrationV1alpha1Api>,
  admissionregistration_v1beta1_api: Box<::apis::AdmissionregistrationV1beta1Api>,
  apiextensions_api: Box<::apis::ApiextensionsApi>,
  apiextensions_v1beta1_api: Box<::apis::ApiextensionsV1beta1Api>,
  apiregistration_api: Box<::apis::ApiregistrationApi>,
  apiregistration_v1_api: Box<::apis::ApiregistrationV1Api>,
  apiregistration_v1beta1_api: Box<::apis::ApiregistrationV1beta1Api>,
  apis_api: Box<::apis::ApisApi>,
  apps_api: Box<::apis::AppsApi>,
  apps_v1_api: Box<::apis::AppsV1Api>,
  apps_v1beta1_api: Box<::apis::AppsV1beta1Api>,
  apps_v1beta2_api: Box<::apis::AppsV1beta2Api>,
  auditregistration_api: Box<::apis::AuditregistrationApi>,
  auditregistration_v1alpha1_api: Box<::apis::AuditregistrationV1alpha1Api>,
  authentication_api: Box<::apis::AuthenticationApi>,
  authentication_v1_api: Box<::apis::AuthenticationV1Api>,
  authentication_v1beta1_api: Box<::apis::AuthenticationV1beta1Api>,
  authorization_api: Box<::apis::AuthorizationApi>,
  authorization_v1_api: Box<::apis::AuthorizationV1Api>,
  authorization_v1beta1_api: Box<::apis::AuthorizationV1beta1Api>,
  autoscaling_api: Box<::apis::AutoscalingApi>,
  autoscaling_v1_api: Box<::apis::AutoscalingV1Api>,
  autoscaling_v2beta1_api: Box<::apis::AutoscalingV2beta1Api>,
  autoscaling_v2beta2_api: Box<::apis::AutoscalingV2beta2Api>,
  batch_api: Box<::apis::BatchApi>,
  batch_v1_api: Box<::apis::BatchV1Api>,
  batch_v1beta1_api: Box<::apis::BatchV1beta1Api>,
  batch_v2alpha1_api: Box<::apis::BatchV2alpha1Api>,
  certificates_api: Box<::apis::CertificatesApi>,
  certificates_v1beta1_api: Box<::apis::CertificatesV1beta1Api>,
  coordination_api: Box<::apis::CoordinationApi>,
  coordination_v1beta1_api: Box<::apis::CoordinationV1beta1Api>,
  core_api: Box<::apis::CoreApi>,
  core_v1_api: Box<::apis::CoreV1Api>,
  events_api: Box<::apis::EventsApi>,
  events_v1beta1_api: Box<::apis::EventsV1beta1Api>,
  extensions_api: Box<::apis::ExtensionsApi>,
  extensions_v1beta1_api: Box<::apis::ExtensionsV1beta1Api>,
  logs_api: Box<::apis::LogsApi>,
  networking_api: Box<::apis::NetworkingApi>,
  networking_v1_api: Box<::apis::NetworkingV1Api>,
  policy_api: Box<::apis::PolicyApi>,
  policy_v1beta1_api: Box<::apis::PolicyV1beta1Api>,
  rbac_authorization_api: Box<::apis::RbacAuthorizationApi>,
  rbac_authorization_v1_api: Box<::apis::RbacAuthorizationV1Api>,
  rbac_authorization_v1alpha1_api: Box<::apis::RbacAuthorizationV1alpha1Api>,
  rbac_authorization_v1beta1_api: Box<::apis::RbacAuthorizationV1beta1Api>,
  scheduling_api: Box<::apis::SchedulingApi>,
  scheduling_v1alpha1_api: Box<::apis::SchedulingV1alpha1Api>,
  scheduling_v1beta1_api: Box<::apis::SchedulingV1beta1Api>,
  settings_api: Box<::apis::SettingsApi>,
  settings_v1alpha1_api: Box<::apis::SettingsV1alpha1Api>,
  storage_api: Box<::apis::StorageApi>,
  storage_v1_api: Box<::apis::StorageV1Api>,
  storage_v1alpha1_api: Box<::apis::StorageV1alpha1Api>,
  storage_v1beta1_api: Box<::apis::StorageV1beta1Api>,
  version_api: Box<::apis::VersionApi>,
}

impl APIClient {
  pub fn new(configuration: Configuration) -> APIClient {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      admissionregistration_api: Box::new(::apis::AdmissionregistrationApiClient::new(rc.clone())),
      admissionregistration_v1alpha1_api: Box::new(::apis::AdmissionregistrationV1alpha1ApiClient::new(rc.clone())),
      admissionregistration_v1beta1_api: Box::new(::apis::AdmissionregistrationV1beta1ApiClient::new(rc.clone())),
      apiextensions_api: Box::new(::apis::ApiextensionsApiClient::new(rc.clone())),
      apiextensions_v1beta1_api: Box::new(::apis::ApiextensionsV1beta1ApiClient::new(rc.clone())),
      apiregistration_api: Box::new(::apis::ApiregistrationApiClient::new(rc.clone())),
      apiregistration_v1_api: Box::new(::apis::ApiregistrationV1ApiClient::new(rc.clone())),
      apiregistration_v1beta1_api: Box::new(::apis::ApiregistrationV1beta1ApiClient::new(rc.clone())),
      apis_api: Box::new(::apis::ApisApiClient::new(rc.clone())),
      apps_api: Box::new(::apis::AppsApiClient::new(rc.clone())),
      apps_v1_api: Box::new(::apis::AppsV1ApiClient::new(rc.clone())),
      apps_v1beta1_api: Box::new(::apis::AppsV1beta1ApiClient::new(rc.clone())),
      apps_v1beta2_api: Box::new(::apis::AppsV1beta2ApiClient::new(rc.clone())),
      auditregistration_api: Box::new(::apis::AuditregistrationApiClient::new(rc.clone())),
      auditregistration_v1alpha1_api: Box::new(::apis::AuditregistrationV1alpha1ApiClient::new(rc.clone())),
      authentication_api: Box::new(::apis::AuthenticationApiClient::new(rc.clone())),
      authentication_v1_api: Box::new(::apis::AuthenticationV1ApiClient::new(rc.clone())),
      authentication_v1beta1_api: Box::new(::apis::AuthenticationV1beta1ApiClient::new(rc.clone())),
      authorization_api: Box::new(::apis::AuthorizationApiClient::new(rc.clone())),
      authorization_v1_api: Box::new(::apis::AuthorizationV1ApiClient::new(rc.clone())),
      authorization_v1beta1_api: Box::new(::apis::AuthorizationV1beta1ApiClient::new(rc.clone())),
      autoscaling_api: Box::new(::apis::AutoscalingApiClient::new(rc.clone())),
      autoscaling_v1_api: Box::new(::apis::AutoscalingV1ApiClient::new(rc.clone())),
      autoscaling_v2beta1_api: Box::new(::apis::AutoscalingV2beta1ApiClient::new(rc.clone())),
      autoscaling_v2beta2_api: Box::new(::apis::AutoscalingV2beta2ApiClient::new(rc.clone())),
      batch_api: Box::new(::apis::BatchApiClient::new(rc.clone())),
      batch_v1_api: Box::new(::apis::BatchV1ApiClient::new(rc.clone())),
      batch_v1beta1_api: Box::new(::apis::BatchV1beta1ApiClient::new(rc.clone())),
      batch_v2alpha1_api: Box::new(::apis::BatchV2alpha1ApiClient::new(rc.clone())),
      certificates_api: Box::new(::apis::CertificatesApiClient::new(rc.clone())),
      certificates_v1beta1_api: Box::new(::apis::CertificatesV1beta1ApiClient::new(rc.clone())),
      coordination_api: Box::new(::apis::CoordinationApiClient::new(rc.clone())),
      coordination_v1beta1_api: Box::new(::apis::CoordinationV1beta1ApiClient::new(rc.clone())),
      core_api: Box::new(::apis::CoreApiClient::new(rc.clone())),
      core_v1_api: Box::new(::apis::CoreV1ApiClient::new(rc.clone())),
      events_api: Box::new(::apis::EventsApiClient::new(rc.clone())),
      events_v1beta1_api: Box::new(::apis::EventsV1beta1ApiClient::new(rc.clone())),
      extensions_api: Box::new(::apis::ExtensionsApiClient::new(rc.clone())),
      extensions_v1beta1_api: Box::new(::apis::ExtensionsV1beta1ApiClient::new(rc.clone())),
      logs_api: Box::new(::apis::LogsApiClient::new(rc.clone())),
      networking_api: Box::new(::apis::NetworkingApiClient::new(rc.clone())),
      networking_v1_api: Box::new(::apis::NetworkingV1ApiClient::new(rc.clone())),
      policy_api: Box::new(::apis::PolicyApiClient::new(rc.clone())),
      policy_v1beta1_api: Box::new(::apis::PolicyV1beta1ApiClient::new(rc.clone())),
      rbac_authorization_api: Box::new(::apis::RbacAuthorizationApiClient::new(rc.clone())),
      rbac_authorization_v1_api: Box::new(::apis::RbacAuthorizationV1ApiClient::new(rc.clone())),
      rbac_authorization_v1alpha1_api: Box::new(::apis::RbacAuthorizationV1alpha1ApiClient::new(rc.clone())),
      rbac_authorization_v1beta1_api: Box::new(::apis::RbacAuthorizationV1beta1ApiClient::new(rc.clone())),
      scheduling_api: Box::new(::apis::SchedulingApiClient::new(rc.clone())),
      scheduling_v1alpha1_api: Box::new(::apis::SchedulingV1alpha1ApiClient::new(rc.clone())),
      scheduling_v1beta1_api: Box::new(::apis::SchedulingV1beta1ApiClient::new(rc.clone())),
      settings_api: Box::new(::apis::SettingsApiClient::new(rc.clone())),
      settings_v1alpha1_api: Box::new(::apis::SettingsV1alpha1ApiClient::new(rc.clone())),
      storage_api: Box::new(::apis::StorageApiClient::new(rc.clone())),
      storage_v1_api: Box::new(::apis::StorageV1ApiClient::new(rc.clone())),
      storage_v1alpha1_api: Box::new(::apis::StorageV1alpha1ApiClient::new(rc.clone())),
      storage_v1beta1_api: Box::new(::apis::StorageV1beta1ApiClient::new(rc.clone())),
      version_api: Box::new(::apis::VersionApiClient::new(rc.clone())),
    }
  }

  pub fn admissionregistration_api(&self) -> &::apis::AdmissionregistrationApi{
    self.admissionregistration_api.as_ref()
  }

  pub fn admissionregistration_v1alpha1_api(&self) -> &::apis::AdmissionregistrationV1alpha1Api{
    self.admissionregistration_v1alpha1_api.as_ref()
  }

  pub fn admissionregistration_v1beta1_api(&self) -> &::apis::AdmissionregistrationV1beta1Api{
    self.admissionregistration_v1beta1_api.as_ref()
  }

  pub fn apiextensions_api(&self) -> &::apis::ApiextensionsApi{
    self.apiextensions_api.as_ref()
  }

  pub fn apiextensions_v1beta1_api(&self) -> &::apis::ApiextensionsV1beta1Api{
    self.apiextensions_v1beta1_api.as_ref()
  }

  pub fn apiregistration_api(&self) -> &::apis::ApiregistrationApi{
    self.apiregistration_api.as_ref()
  }

  pub fn apiregistration_v1_api(&self) -> &::apis::ApiregistrationV1Api{
    self.apiregistration_v1_api.as_ref()
  }

  pub fn apiregistration_v1beta1_api(&self) -> &::apis::ApiregistrationV1beta1Api{
    self.apiregistration_v1beta1_api.as_ref()
  }

  pub fn apis_api(&self) -> &::apis::ApisApi{
    self.apis_api.as_ref()
  }

  pub fn apps_api(&self) -> &::apis::AppsApi{
    self.apps_api.as_ref()
  }

  pub fn apps_v1_api(&self) -> &::apis::AppsV1Api{
    self.apps_v1_api.as_ref()
  }

  pub fn apps_v1beta1_api(&self) -> &::apis::AppsV1beta1Api{
    self.apps_v1beta1_api.as_ref()
  }

  pub fn apps_v1beta2_api(&self) -> &::apis::AppsV1beta2Api{
    self.apps_v1beta2_api.as_ref()
  }

  pub fn auditregistration_api(&self) -> &::apis::AuditregistrationApi{
    self.auditregistration_api.as_ref()
  }

  pub fn auditregistration_v1alpha1_api(&self) -> &::apis::AuditregistrationV1alpha1Api{
    self.auditregistration_v1alpha1_api.as_ref()
  }

  pub fn authentication_api(&self) -> &::apis::AuthenticationApi{
    self.authentication_api.as_ref()
  }

  pub fn authentication_v1_api(&self) -> &::apis::AuthenticationV1Api{
    self.authentication_v1_api.as_ref()
  }

  pub fn authentication_v1beta1_api(&self) -> &::apis::AuthenticationV1beta1Api{
    self.authentication_v1beta1_api.as_ref()
  }

  pub fn authorization_api(&self) -> &::apis::AuthorizationApi{
    self.authorization_api.as_ref()
  }

  pub fn authorization_v1_api(&self) -> &::apis::AuthorizationV1Api{
    self.authorization_v1_api.as_ref()
  }

  pub fn authorization_v1beta1_api(&self) -> &::apis::AuthorizationV1beta1Api{
    self.authorization_v1beta1_api.as_ref()
  }

  pub fn autoscaling_api(&self) -> &::apis::AutoscalingApi{
    self.autoscaling_api.as_ref()
  }

  pub fn autoscaling_v1_api(&self) -> &::apis::AutoscalingV1Api{
    self.autoscaling_v1_api.as_ref()
  }

  pub fn autoscaling_v2beta1_api(&self) -> &::apis::AutoscalingV2beta1Api{
    self.autoscaling_v2beta1_api.as_ref()
  }

  pub fn autoscaling_v2beta2_api(&self) -> &::apis::AutoscalingV2beta2Api{
    self.autoscaling_v2beta2_api.as_ref()
  }

  pub fn batch_api(&self) -> &::apis::BatchApi{
    self.batch_api.as_ref()
  }

  pub fn batch_v1_api(&self) -> &::apis::BatchV1Api{
    self.batch_v1_api.as_ref()
  }

  pub fn batch_v1beta1_api(&self) -> &::apis::BatchV1beta1Api{
    self.batch_v1beta1_api.as_ref()
  }

  pub fn batch_v2alpha1_api(&self) -> &::apis::BatchV2alpha1Api{
    self.batch_v2alpha1_api.as_ref()
  }

  pub fn certificates_api(&self) -> &::apis::CertificatesApi{
    self.certificates_api.as_ref()
  }

  pub fn certificates_v1beta1_api(&self) -> &::apis::CertificatesV1beta1Api{
    self.certificates_v1beta1_api.as_ref()
  }

  pub fn coordination_api(&self) -> &::apis::CoordinationApi{
    self.coordination_api.as_ref()
  }

  pub fn coordination_v1beta1_api(&self) -> &::apis::CoordinationV1beta1Api{
    self.coordination_v1beta1_api.as_ref()
  }

  pub fn core_api(&self) -> &::apis::CoreApi{
    self.core_api.as_ref()
  }

  pub fn core_v1_api(&self) -> &::apis::CoreV1Api{
    self.core_v1_api.as_ref()
  }

  pub fn events_api(&self) -> &::apis::EventsApi{
    self.events_api.as_ref()
  }

  pub fn events_v1beta1_api(&self) -> &::apis::EventsV1beta1Api{
    self.events_v1beta1_api.as_ref()
  }

  pub fn extensions_api(&self) -> &::apis::ExtensionsApi{
    self.extensions_api.as_ref()
  }

  pub fn extensions_v1beta1_api(&self) -> &::apis::ExtensionsV1beta1Api{
    self.extensions_v1beta1_api.as_ref()
  }

  pub fn logs_api(&self) -> &::apis::LogsApi{
    self.logs_api.as_ref()
  }

  pub fn networking_api(&self) -> &::apis::NetworkingApi{
    self.networking_api.as_ref()
  }

  pub fn networking_v1_api(&self) -> &::apis::NetworkingV1Api{
    self.networking_v1_api.as_ref()
  }

  pub fn policy_api(&self) -> &::apis::PolicyApi{
    self.policy_api.as_ref()
  }

  pub fn policy_v1beta1_api(&self) -> &::apis::PolicyV1beta1Api{
    self.policy_v1beta1_api.as_ref()
  }

  pub fn rbac_authorization_api(&self) -> &::apis::RbacAuthorizationApi{
    self.rbac_authorization_api.as_ref()
  }

  pub fn rbac_authorization_v1_api(&self) -> &::apis::RbacAuthorizationV1Api{
    self.rbac_authorization_v1_api.as_ref()
  }

  pub fn rbac_authorization_v1alpha1_api(&self) -> &::apis::RbacAuthorizationV1alpha1Api{
    self.rbac_authorization_v1alpha1_api.as_ref()
  }

  pub fn rbac_authorization_v1beta1_api(&self) -> &::apis::RbacAuthorizationV1beta1Api{
    self.rbac_authorization_v1beta1_api.as_ref()
  }

  pub fn scheduling_api(&self) -> &::apis::SchedulingApi{
    self.scheduling_api.as_ref()
  }

  pub fn scheduling_v1alpha1_api(&self) -> &::apis::SchedulingV1alpha1Api{
    self.scheduling_v1alpha1_api.as_ref()
  }

  pub fn scheduling_v1beta1_api(&self) -> &::apis::SchedulingV1beta1Api{
    self.scheduling_v1beta1_api.as_ref()
  }

  pub fn settings_api(&self) -> &::apis::SettingsApi{
    self.settings_api.as_ref()
  }

  pub fn settings_v1alpha1_api(&self) -> &::apis::SettingsV1alpha1Api{
    self.settings_v1alpha1_api.as_ref()
  }

  pub fn storage_api(&self) -> &::apis::StorageApi{
    self.storage_api.as_ref()
  }

  pub fn storage_v1_api(&self) -> &::apis::StorageV1Api{
    self.storage_v1_api.as_ref()
  }

  pub fn storage_v1alpha1_api(&self) -> &::apis::StorageV1alpha1Api{
    self.storage_v1alpha1_api.as_ref()
  }

  pub fn storage_v1beta1_api(&self) -> &::apis::StorageV1beta1Api{
    self.storage_v1beta1_api.as_ref()
  }

  pub fn version_api(&self) -> &::apis::VersionApi{
    self.version_api.as_ref()
  }


}
