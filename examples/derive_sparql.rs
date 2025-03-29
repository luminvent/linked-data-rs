use linked_data::{SparqlSerialize, SparqlQueryGenerator};

#[derive(SparqlSerialize)]
#[ld(prefix("ex" = "http://example.org/"))]
struct Book {
	#[ld("dc:hasTitle")]
	title: String,
}

fn main() {
	let query = <Book as SparqlQueryGenerator::<(), ()>>::generate_list_query();
	println!("{}", query);
}
