use rdf_types::generator::Blank;

use rdf_types::interpretation::{IdInterpretationMut, TermInterpretationMut, WithGenerator};
use rdf_types::{Generator, Quad, RdfDisplay, Triple};

use crate::sparql::interpretation::SparqlInterpretation;
use crate::{to_quads, LinkedData};

pub mod interpretation;

#[derive(Clone, Debug, Default)]
pub struct SparqlVariable(pub String);

impl RdfDisplay for SparqlVariable {
	fn rdf_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

pub fn to_sparql<G>(value: &impl LinkedData<WithGenerator<Blank>>) -> String
where
	G: Generator,
{
	let mut interpretation = SparqlInterpretation::default();

	let pattern_lines: Vec<String> = to_quads(Blank::new(), value)
		.expect("RDF serialization failed")
		.into_iter()
		.map(|Quad(s, p, o, _)| {
			Triple(
				interpretation.interpret_id(s),
				p,
				interpretation.interpret_term(o),
			)
		})
		.map(|triple| format!("    {} .", triple))
		.collect();

	let pattern = pattern_lines.join("\n");

	format!("SELECT {{\n{}\n}}\nWHERE {{\n{}\n}}", pattern, pattern)
}

// #[derive(Clone, Debug)]
// pub struct Variable {
//     term: Term
//     variable:
// }

// impl LinkedData for Var {
// 	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
// 	where
// 		S: linked_data::Visitor<(), ()>,
// 	{
// 		println!("hiii");
// 		visitor.end()
// 	}
// }
//
// impl<I, V> LinkedDataSubject<I, V> for Var
// where
// 	I: Interpretation<Resource = Var>,
// 	V: Vocabulary,
// {
// 	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 	where
// 		S: linked_data::SubjectVisitor<I, V>,
// 	{
// 		todo!()
// 	}
// }
//
// impl<I, V> LinkedDataResource<I, V> for Var
// where
// 	I: Interpretation<Resource = Var>,
// 	V: Vocabulary,
// {
// 	fn interpretation(
// 		&self,
// 		vocabulary: &mut V,
// 		interpretation: &mut I,
// 	) -> linked_data::ResourceInterpretation<I, V> {
// 		ResourceInterpretation::Interpreted(self)
// 	}
// }
//
// impl<V> LinkedDataPredicateObjects<I, V> for Var
// where
// 	V: Vocabulary,
// {
// 	fn visit_objects<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
// 	where
// 		S: linked_data::PredicateObjectsVisitor<MyInter, V>,
// 	{
// 		println!("hooo");
// 		visitor.object(self)?;
// 		visitor.end()
// 	}
// }
