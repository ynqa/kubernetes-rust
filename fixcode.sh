#!/bin/sh -e

OPT=gsed
OUTPUT_DIR=kubernetes-apis

JSON_SCHEMA_PROPS=${OUTPUT_DIR}/src/models/io_k8s_apiextensions_apiserver_pkg_apis_apiextensions_v1beta1_json_schema_props.rs
CORE_V1_API=${OUTPUT_DIR}/src/apis/core_v1_api.rs
HTTP_GET_ACTION=${OUTPUT_DIR}/src/models/io_k8s_api_core_v1_http_get_action.rs

# avoid recursive type has infinite size
${OPT} -i 's/not: Option<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>/not: Option<Box<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>>/g' "${JSON_SCHEMA_PROPS}"
${OPT} -i 's/not: Option<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>/not: Option<Box<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>>/g' "${JSON_SCHEMA_PROPS}"
${OPT} -i 's/not: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps/not: Box<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>/g' "${JSON_SCHEMA_PROPS}"
${OPT} -i 's/not: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps/not: Box<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>/g' "${JSON_SCHEMA_PROPS}"
${OPT} -i 's/pub fn not(\&self) -> Option<\&::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>/pub fn not(\&self) -> Option<\&Box<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>>/g' "${JSON_SCHEMA_PROPS}"
${OPT} -i 's/pub fn not(\&self) -> Option<\&::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>/pub fn not(\&self) -> Option<\&Box<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1JsonSchemaProps>>/g' "${JSON_SCHEMA_PROPS}"

# delete apis to request `options`
${OPT} -i '/fn connect_core_v1_options_namespaced_pod_proxy(&self, name: &str, namespace: &str, path: &str) -> Result<String, Error>;/d' "${CORE_V1_API}"
${OPT} -i '/fn connect_core_v1_options_namespaced_pod_proxy_with_path(&self, name: &str, namespace: &str, path: &str, path2: &str) -> Result<String, Error>;/d' "${CORE_V1_API}"
${OPT} -i '/fn connect_core_v1_options_namespaced_service_proxy(&self, name: &str, namespace: &str, path: &str) -> Result<String, Error>;/d' "${CORE_V1_API}" 
${OPT} -i '/fn connect_core_v1_options_namespaced_service_proxy_with_path(&self, name: &str, namespace: &str, path: &str, path2: &str) -> Result<String, Error>;/d' "${CORE_V1_API}"
${OPT} -i '/fn connect_core_v1_options_node_proxy(&self, name: &str, path: &str) -> Result<String, Error>;/d' "${CORE_V1_API}"
${OPT} -i '/fn connect_core_v1_options_node_proxy_with_path(&self, name: &str, path: &str, path2: &str) -> Result<String, Error>;/d' "${CORE_V1_API}"

${OPT} -i '/connect_core_v1_options_namespaced_pod_proxy/,+36d' "${CORE_V1_API}"
${OPT} -i '/connect_core_v1_options_namespaced_pod_proxy_with_path/,+36d' "${CORE_V1_API}"
${OPT} -i '/connect_core_v1_options_namespaced_service_proxy/,+36d' "${CORE_V1_API}"
${OPT} -i '/connect_core_v1_options_namespaced_service_proxy_with_path/,+36d' "${CORE_V1_API}"
${OPT} -i '/connect_core_v1_options_node_proxy/,+36d' "${CORE_V1_API}"
${OPT} -i '/connect_core_v1_options_node_proxy_with_path/,+36d' "${CORE_V1_API}"

# port type as i32 on http_get_action
${OPT} -i 's/port: String/port: i32/g' "${HTTP_GET_ACTION}"
${OPT} -i 's/port(\&self) -> \&String/port(\&self) -> \&i32/g' "${HTTP_GET_ACTION}"
