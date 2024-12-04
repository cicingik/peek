use k8s_openapi::api::core::v1::{ConfigMap, Secret};
use kube::{api::ListParams, config::KubeConfigOptions, Api, Client, ResourceExt};
use tracing::info;

use crate::{entities::ResourceConfig, errors::AppError, options::Options};

pub async fn new_k8s_client(context: Option<String>) -> Result<Client, AppError> {
	let mut options = KubeConfigOptions {
		context: context.clone(),
		cluster: None,
		user: None,
	};

	if context.is_some() {
		options.context = context;
	}

	let mut config = kube::Config::from_kubeconfig(&options).await.map_err(|cause| {
		AppError::KubeConfigError {
			context: "create the kube client config".to_string(),
			cause,
		}
	})?;

	config.connect_timeout = Some(std::time::Duration::from_secs(30));
	config.read_timeout = Some(std::time::Duration::from_secs(15));

	Client::try_from(config).map_err(|cause| AppError::KubeError {
		context: "create the kube client".to_string(),
		cause,
	})
}

pub async fn get_configmaps(
	client: Client,
	out: &mut Vec<ResourceConfig>,
	opts: &mut Options,
) -> Result<(), AppError> {
	if opts.namespace.is_none() {
		opts.namespace = Some(String::from(client.default_namespace()))
	}

	if opts.all {
		opts.namespace = Some(String::new());
	}

	let cm_api: Api<ConfigMap> = Api::namespaced(client, opts.namespace.clone().as_ref().unwrap());
	let mut lp = ListParams::default();
	if opts.name.is_some() {
		let mut resource_name = String::from("name=");
		resource_name.push_str(opts.name.as_ref().unwrap());
		lp.label_selector = Some(resource_name);
	}

	let configmaps = cm_api.list(&lp).await.map_err(|cause| AppError::KubeError {
		context: "list configmap".to_string(),
		cause,
	})?;

	for configmap in configmaps {
		info!("peeking into {} configmap", configmap.clone().name_any());
		if configmap.data.is_some() {
			let namespace = configmap.clone().metadata.namespace.unwrap_or_default();
			let cmaps_name = configmap.name_any();

			for (key, value) in configmap.data.unwrap() {
				let mut found: bool = false;

				if opts.key.is_some() && key.contains(&opts.key.as_ref().unwrap().to_owned()) {
					found = true;
				}

				if opts.value.is_some() && value.contains(&opts.value.as_ref().unwrap().to_owned())
				{
					found = true;
				}

				if found {
					let tmp = ResourceConfig {
						kind: "Configmaps".to_string(),
						name: cmaps_name.clone(),
						namespace: namespace.clone(),
						key,
						value,
					};

					out.push(tmp);
				}
			}
		}
	}

	Ok(())
}

pub async fn get_secrets(
	client: Client,
	out: &mut Vec<ResourceConfig>,
	opts: &mut Options,
) -> Result<(), AppError> {
	if opts.namespace.is_none() {
		opts.namespace = Some(String::from(client.default_namespace()))
	}

	if opts.all {
		opts.namespace = Some(String::new());
	}

	let sc_api: Api<Secret> = Api::namespaced(client, opts.namespace.clone().as_ref().unwrap());
	let mut lp = ListParams::default();
	if opts.name.is_some() {
		let mut resource_name = String::from("name=");
		resource_name.push_str(opts.name.as_ref().unwrap());
		lp.label_selector = Some(resource_name);
	}

	let secrets = sc_api.list(&lp).await.map_err(|cause| AppError::KubeError {
		context: "list secret".to_string(),
		cause,
	})?;

	for secret in secrets {
		info!("peeking into {} secret", secret.clone().name_any());
		if secret.data.is_some() {
			let namespace = secret.clone().metadata.namespace.unwrap_or_default();
			let secret_name = secret.name_any();

			for (key, v) in secret.data.unwrap() {
				let value = String::from_utf8(v.clone().0).unwrap_or_default();
				let mut found: bool = false;

				if opts.key.is_some() && key.contains(&opts.key.as_ref().unwrap().to_owned()) {
					found = true;
				}

				if opts.value.is_some() && value.contains(&opts.value.as_ref().unwrap().to_owned())
				{
					found = true;
				}

				if found {
					let tmp = ResourceConfig {
						kind: "Secrets".to_string(),
						name: secret_name.clone(),
						namespace: namespace.clone(),
						key,
						value,
					};

					out.push(tmp);
				}
			}
		}
	}

	Ok(())
}
