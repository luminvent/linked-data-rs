use iref::IriBuf;
use linked_data_next::{to_quads, LinkedDataDeserializeSubject};
use linked_data_next_derive::{Deserialize, Serialize};
use rdf_types::dataset::IndexedBTreeDataset;
use rdf_types::{generator, Term};

#[derive(Debug, Serialize, Deserialize)]
#[ld(prefix("ex" = "http://example.org/"))]
struct Foo {
	#[ld(id)]
	id: IriBuf,
	#[ld("ex:name")]
	name: String,
}

fn main() {
	let mut dataset = IndexedBTreeDataset::new();

	let id_1 = IriBuf::new("http://example.org/john".to_string()).unwrap();

	let value_1 = Foo {
		id: id_1.clone(),
		name: "John Smith".to_string(),
	};

	to_quads(generator::Blank::new(), &value_1)
		.expect("RDF serialization failed")
		.into_iter()
		.for_each(|rdf_quad| {
			let quad = rdf_types::Quad(
				Term::Id(rdf_quad.0),
				Term::iri(rdf_quad.1),
				rdf_quad.2.into(),
				rdf_quad.3.map(Term::Id),
			);

			dataset.insert(quad);
		});

	let id_2 = IriBuf::new("http://example.org/joe".to_string()).unwrap();

	let value_2 = Foo {
		id: id_2.clone(),
		name: "Joe Dalton".to_string(),
	};

	to_quads(generator::Blank::new(), &value_2)
		.expect("RDF serialization failed")
		.into_iter()
		.for_each(|rdf_quad| {
			let quad = rdf_types::Quad(
				Term::Id(rdf_quad.0),
				Term::iri(rdf_quad.1),
				rdf_quad.2.into(),
				rdf_quad.3.map(Term::Id),
			);

			dataset.insert(quad);
		});

	let resources = [Term::iri(id_1), Term::iri(id_2)];

	let objects = Foo::deserialize_subjects(&(), &(), &dataset, None, resources).unwrap();

	println!("{objects:#?}");
}
