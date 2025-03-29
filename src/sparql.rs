use rdf_types::{Interpretation, RdfDisplay, Vocabulary};

pub mod interpretation;

#[derive(Clone, Debug, Default)]
pub struct SparqlVariable(pub String);

impl RdfDisplay for SparqlVariable {
	fn rdf_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

pub trait SparqlQueryGenerator<I: Interpretation, V: Vocabulary> {
	fn generate_list_query() -> String;
	fn generate_get_query(id: &str) -> String;
}
