use linked_data_next::{Deserialize, Serialize, to_quads};
use rdf_types::{RdfDisplay, generator};
use std::collections::HashSet;

#[derive(Deserialize, Serialize)]
#[ld(prefix("ex" = "http://example.org/"))]
struct Foo {
	#[ld("ex:name")]
	name: HashSet<String>,
}

fn main() {
	let value = Foo {
		name: HashSet::from(["John Smith".to_string()]),
	};

	let quads = to_quads(generator::Blank::new(), &value).expect("RDF serialization failed");

	for quad in quads {
		println!("{} .", quad.rdf_display())
	}
}
