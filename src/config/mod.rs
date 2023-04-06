mod apis;
mod exec;
mod incluster_config;
mod kube_config;
mod utils;

use base64;
use failure::{format_err, Error};
use reqwest::{header, Certificate, Client, Identity};

use self::kube_config::KubeConfigLoader;

/// Configuration stores kubernetes path and client for requests.
pub struct Configuration {
    pub base_path: String,
    pub client: Client,
}

impl Configuration {
    pub fn new(base_path: String, client: Client) -> Self {
        Configuration {
            base_path: base_path.to_owned(),
            client: client,
        }
    }
}

/// Returns a config includes authentication and cluster information from kubeconfig file.
///
/// # Example
/// ```no_run
/// use kubernetes::config;
///
/// let kubeconfig = config::load_kube_config()
///     .expect("failed to load kubeconfig");
/// ```
pub fn load_kube_config() -> Result<Configuration, Error> {
    load_kube_config_with(Default::default())
}

/// ConfigOptions stores options used when loading kubeconfig file.
#[derive(Default)]
pub struct ConfigOptions {
    pub context: Option<String>,
    pub cluster: Option<String>,
    pub user: Option<String>,
}

/// Returns a config includes authentication and cluster information from kubeconfig file.
///
/// # Example
/// ```no_run
/// use kubernetes::config;
///
/// let kubeconfig = config::load_kube_config()
///     .expect("failed to load kubeconfig");
/// ```
pub fn load_kube_config_with(options: ConfigOptions) -> Result<Configuration, Error> {
    let kubeconfig = utils::kubeconfig_path()
        .or_else(utils::default_kube_path)
        .ok_or(format_err!("Unable to load kubeconfig"))?;

    let loader =
        KubeConfigLoader::load(kubeconfig, options.context, options.cluster, options.user)?;
    let token = match &loader.user.token {
        Some(token) => Some(token.clone()),
        None => {
            if let Some(exec) = &loader.user.exec {
                let creds = exec::auth_exec(exec)?;
                let status = creds
                    .status
                    .ok_or(format_err!("exec-plugin response did not contain a status"))?;
                status.token
            } else {
                None
            }
        }
    };

    let mut client_builder = Client::builder();

    if let Some(ca) = loader.ca() {
        let req_ca = Certificate::from_der(&ca?.to_der()?)?;
        client_builder = client_builder.add_root_certificate(req_ca);
    }
    match loader.p12(" ") {
        Ok(p12) => {
            let req_p12 = Identity::from_pkcs12_der(&p12.to_der()?, " ")?;
            client_builder = client_builder.identity(req_p12);
        }
        Err(_) => {
            // last resort only if configs ask for it, and no client certs
            if let Some(true) = loader.cluster.insecure_skip_tls_verify {
                client_builder = client_builder.danger_accept_invalid_certs(true);
            }
        }
    }

    let mut headers = header::HeaderMap::new();

    match (
        utils::data_or_file(&token, &loader.user.token_file),
        (loader.user.username, loader.user.password),
    ) {
        (Ok(token), _) => {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
        }
        (_, (Some(u), Some(p))) => {
            let encoded = base64::encode(&format!("{}:{}", u, p));
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Basic {}", encoded))?,
            );
        }
        _ => {}
    }

    let client_builder = client_builder.default_headers(headers);

    Ok(Configuration::new(
        loader.cluster.server,
        client_builder.build()?,
    ))
}

/// Returns a config which is used by clients within pods on kubernetes.
/// It will return an error if called from out of kubernetes cluster.
///
/// # Example
/// ```no_run
/// use kubernetes::config;
///
/// let kubeconfig = config::incluster_config()
///     .expect("failed to load incluster config");
/// ```
pub fn incluster_config() -> Result<Configuration, Error> {
    let server = incluster_config::kube_server().ok_or(format_err!(
        "Unable to load incluster config, {} and {} must be defined",
        incluster_config::SERVICE_HOSTENV,
        incluster_config::SERVICE_PORTENV
    ))?;

    let ca = incluster_config::load_cert()?;
    let req_ca = Certificate::from_der(&ca.to_der()?)?;

    let token = incluster_config::load_token()?;
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let client_builder = Client::builder()
        .add_root_certificate(req_ca)
        .default_headers(headers);

    Ok(Configuration::new(server, client_builder.build()?))
}
