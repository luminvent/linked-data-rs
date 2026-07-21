use rdf_types::{
	Id, Interpretation, RDF_FIRST, RDF_NIL, RDF_REST, Term, Vocabulary,
	vocabulary::IriVocabularyMut,
};

use crate::{
	CowRdfTerm, LinkedDataPredicateObjects, LinkedDataResource, LinkedDataSubject,
	PredicateObjectsVisitor, ResourceInterpretation, SubjectVisitor,
};

/// Binds a single value as the object of a predicate, regardless of whether
/// that value implements [`LinkedDataPredicateObjects`] itself.
struct Single<'a, T>(&'a T);

impl<I: Interpretation, V: Vocabulary, T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>>
	LinkedDataPredicateObjects<I, V> for Single<'_, T>
{
	fn visit_objects<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
	where
		S: PredicateObjectsVisitor<I, V>,
	{
		visitor.object(self.0)?;
		visitor.end()
	}
}

/// `rdf:List` representation of a slice.
///
/// An empty slice is serialized as the `rdf:nil` resource. A non-empty
/// slice is serialized as a chain of blank nodes linked by `rdf:first` and
/// `rdf:rest`, terminated by `rdf:nil`.
pub(crate) struct RdfList<'a, T>(pub &'a [T]);

impl<I: Interpretation, V: Vocabulary + IriVocabularyMut, T> LinkedDataResource<I, V>
	for RdfList<'_, T>
{
	fn interpretation(
		&self,
		vocabulary: &mut V,
		_interpretation: &mut I,
	) -> ResourceInterpretation<'_, I, V> {
		if self.0.is_empty() {
			ResourceInterpretation::Uninterpreted(Some(CowRdfTerm::Owned(Term::Id(Id::Iri(
				vocabulary.insert(RDF_NIL),
			)))))
		} else {
			ResourceInterpretation::Uninterpreted(None)
		}
	}
}

impl<
	I: Interpretation,
	V: Vocabulary + IriVocabularyMut,
	T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
> LinkedDataSubject<I, V> for RdfList<'_, T>
{
	fn visit_subject<S>(&self, mut serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		if let Some((first, rest)) = self.0.split_first() {
			serializer.predicate(RDF_FIRST, &Single(first))?;
			serializer.predicate(RDF_REST, &RdfList(rest))?;
		}

		serializer.end()
	}
}

impl<
	I: Interpretation,
	V: Vocabulary + IriVocabularyMut,
	T: LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
> LinkedDataPredicateObjects<I, V> for RdfList<'_, T>
{
	fn visit_objects<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
	where
		S: PredicateObjectsVisitor<I, V>,
	{
		visitor.object(self)?;
		visitor.end()
	}
}
