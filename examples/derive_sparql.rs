use linked_data::{to_sparql, Deserialize, Serialize};
use rdf_types::generator::Blank;

#[derive(Serialize, Deserialize)]
#[ld(prefix("ex" = "http://example.org/"))]
struct Book {
	#[ld("dc:title")]
	title: String,
}

// #[derive(Serialize, Deserialize)]
// #[ld(prefix("ex" = "http://example.org/"))]
// struct SparqlBook {
// 	#[ld("dc:title")]
// 	title: Variable,
// }

fn main() {
	let book = Book {
		title: "A book".to_string(),
	};

	println!("{}", to_sparql::<Blank>(&book));

	// let mut interpretation = SparqlInterpretation::default();

	// let quads = to_quads(Blank::new(), &book).expect("RDF serialization failed");
	//
	// quads
	// 	.into_iter()
	// 	.map(|Quad(s, p, o, _)| {
	// 		Triple(
	// 			interpretation.interpret_id(s),
	// 			p,
	// 			interpretation.interpret_term(o),
	// 		)
	// 	})
	// 	.for_each(|triple| println!("{:?} .", triple));
	//
	// let book = SparqlBook {
	// 	title: VariableOrTerm::Variable(SparqlVariable("?a".to_string())),
	// };
	//
	// let mut interpretation = MyInter::default();
	// let quads = to_interpreted_quads(&mut (), &mut interpretation, &value)
	// 	.expect("RDF serialization failed");
	//
	// for quad in quads {
	// 	println!("{:?} .", quad)
	// }

	// 	let mut interpretation = rdf_types::interpretation::WithGenerator::new((), Blank::new());
}
