use std::collections::HashMap;

use iref::IriBuf;
use rdf_types::interpretation::{
	BlankIdInterpretationMut, IriInterpretationMut, LiteralInterpretationMut,
};
use rdf_types::{
	BlankIdBuf, FromBlankId, FromIri, Interpretation, InterpretationMut, Literal, Term,
};

use crate::sparql::SparqlVariable;

#[derive(Default, Debug)]
pub struct SparqlInterpretation {
	pub variables: HashMap<Term, SparqlVariable>,
	pub next_var_index: usize,
}

impl Interpretation for SparqlInterpretation {
	type Resource = SparqlVariable;
}

impl SparqlInterpretation {
	pub fn add_term(&mut self, term: Term) -> <Self as Interpretation>::Resource {
		match self.variables.get(&term) {
			Some(sparql_variable) => sparql_variable.clone(),
			None => {
				let sparql_variable = self.new_resource(&mut ());
				self.variables.insert(term, sparql_variable.clone());
				sparql_variable
			}
		}
	}
}

impl<V> InterpretationMut<V> for SparqlInterpretation {
	fn new_resource(&mut self, _vocabulary: &mut V) -> Self::Resource {
		// TODO Case index > 26
		let letter = (b'a' + (self.next_var_index as u8 % 26)) as char;
		self.next_var_index += 1;
		SparqlVariable(format!("?{}", letter))
	}
}

impl BlankIdInterpretationMut for SparqlInterpretation {
	fn interpret_blank_id(&mut self, blank_id: BlankIdBuf) -> Self::Resource {
		self.add_term(Term::from_blank(blank_id))
	}
}

impl IriInterpretationMut for SparqlInterpretation {
	fn interpret_iri(&mut self, iri: IriBuf) -> Self::Resource {
		self.add_term(Term::from_iri(iri))
	}
}

impl LiteralInterpretationMut for SparqlInterpretation {
	fn interpret_literal(&mut self, literal: Literal) -> Self::Resource {
		self.add_term(Term::Literal(literal))
	}
}

// impl LinkedData<VariableInterpretation> for Variable {
// 	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
// 	where
// 		S: linked_data::Visitor<VariableInterpretation, ()>,
// 	{
// 		visitor.end()
// 	}
// }
//
