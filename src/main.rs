use tracing::Level;
use tracing_subscriber::filter::LevelFilter;

mod entities;
mod errors;
mod k8s;
mod options;
mod render;

async fn peeking(
	resources: &mut Vec<entities::ResourceConfig>,
	options: &mut options::Options,
) -> Result<(), errors::AppError> {
	let client = k8s::new_k8s_client(options.context.clone()).await?;

	if options.resource.is_some() {
		let mut opts = options.clone();

		match opts.clone().resource.unwrap() {
			options::Resources::Configmaps => {
				k8s::get_configmaps(client, resources, &mut opts).await?
			}
			options::Resources::Secrets => k8s::get_secrets(client, resources, &mut opts).await?,
		}
	} else {
		let mut opts = options.clone();

		k8s::get_configmaps(client.clone(), resources, &mut opts).await?;
		k8s::get_secrets(client.clone(), resources, &mut opts).await?;
	}

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), errors::AppError> {
	tracing_subscriber::fmt()
		.with_max_level(LevelFilter::from_level(Level::WARN))
		.init();

	let mut opts: options::Options = clap::Parser::parse();
	let mut resources: Vec<entities::ResourceConfig> = vec![];

	peeking(&mut resources, &mut opts).await?;
	render::output(&resources, &mut opts).await;

	Ok(())
}
