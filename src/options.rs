use clap::Parser;

/// Peeking your key or/and value from configmap or/and secret
#[derive(Parser, Clone, PartialEq)]
pub struct Options {
	/// Name of the kubeconfig context target. For override current context
	#[arg(long, short)]
	pub context: Option<String>,

	/// Name of namespace target. Ignored if flag -A is present
	#[arg(long, short)]
	pub namespace: Option<String>,

	/// Key name data resources
	#[arg(long, short)]
	pub key: Option<String>,

	/// Value data resources
	#[arg(long, short)]
	pub value: Option<String>,

	/// If present, list the requested resource(s) across all namespaces
	#[arg(long, short = 'A')]
	pub all: bool,

	/// Kind of resource
	pub resource: Option<Resources>,

	/// Resource name
	pub name: Option<String>,

	/// Format output
	#[arg(long, short, default_value_t = OutputMode::Table)]
	pub output: OutputMode,
}

#[derive(Clone, PartialEq, Eq, Debug, clap::ValueEnum)]
pub enum Resources {
	Configmaps,
	Secrets,
}

#[derive(Clone, PartialEq, Eq, Debug, clap::ValueEnum)]
pub enum OutputMode {
	Table,
	Yaml,
	Json,
}

impl OutputMode {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Table => "table",
			Self::Yaml => "yaml",
			Self::Json => "json",
		}
	}
}

impl std::fmt::Display for OutputMode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.pad(self.as_str())
	}
}
