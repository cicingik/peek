use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum AppError {
	#[error("Failed to perform action using {context}")]
	KubeError { context: String, cause: kube::Error },

	#[error("Failed load config for {context}")]
	KubeConfigError {
		context: String,
		cause: kube::config::KubeconfigError,
	},
}
