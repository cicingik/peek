use std::result::Result::Ok;

use prettytable::{cell, format, format::TableFormat, row, Row, Table};
use tracing::error;

use crate::{entities, options};

fn get_table_format() -> TableFormat {
	format::FormatBuilder::new()
		.separators(
			&[
				format::LinePosition::Top,
				format::LinePosition::Bottom,
				format::LinePosition::Title,
			],
			format::LineSeparator::new('=', '+', '+', '+'),
		)
		.separator(format::LinePosition::Intern, format::LineSeparator::new('-', '+', '+', '+'))
		.column_separator('|')
		.borders('|')
		.padding(2, 2)
		.indent(2)
		.build()
}

async fn render_as_table(data: &Vec<entities::ResourceConfig>) {
	let mut table = Table::new();
	table.set_format(get_table_format());

	let row_titles = row![
		bc->"Kind",
		bc->"Namespace",
		bc->"Name",
		bc->"Key",
		bc->"Value",
	];
	table.set_titles(row_titles);

	for d in data {
		#[allow(clippy::string_to_string)]
		let mut prefix_value = String::from(d.value.clone());
		if prefix_value.len() > 64 {
			let (prefix, _) = d.value.split_at(64);
			prefix_value = String::from(prefix);
			prefix_value.push_str(" ...");
		}

		let c = vec![
			cell!(d.kind),
			cell!(d.namespace),
			cell!(d.name),
			cell!(d.key),
			cell!(prefix_value),
		];
		table.add_row(Row::new(c));
	}

	table.printstd();
}

pub async fn output(data: &Vec<entities::ResourceConfig>, opts: &mut options::Options) {
	match opts.output {
		options::OutputMode::Table => render_as_table(data).await,
		options::OutputMode::Yaml => {
			match serde_yaml::to_string(data) {
				Ok(r) => println!("{}", r),
				Err(err) => {
					error!("failed serialize to yaml: {:?}", err)
				}
			};
		}
		options::OutputMode::Json => {
			match serde_json::to_string(data) {
				Ok(r) => println!("{}", r),
				Err(err) => {
					error!("failed serialize to json: {:?}", err)
				}
			};
		}
	}
}
