struct SparqlDomain;

impl<V> Domain<VariableInterpretation, V> for SparqlDomain
where
	V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut,
	V::Iri: Clone,
	V::BlankId: Clone,
	V::Literal: Clone,
{
    type Subject;

    type Predicate;

    type Object;

    type ObjectRef<'a>
	    where
		    <V>::Iri: 'a,
		    <V>::BlankId: 'a,
		    <V>::Literal: 'a,
		    <VariableInterpretation as Interpretation>::Resource: 'a;

    fn subject(
		    &mut self,
		    vocabulary: &mut V,
		    interpretation: &mut VariableInterpretation,
		    value: ResourceInterpretation<VariableInterpretation, V>,
	    ) -> Result<Self::Subject, linked_data::IntoQuadsError> {
        todo!()
    }

    fn predicate(
		    &mut self,
		    vocabulary: &mut V,
		    interpretation: &mut VariableInterpretation,
		    value: ResourceInterpretation<VariableInterpretation, V>,
	    ) -> Result<Self::Predicate, linked_data::IntoQuadsError> {
        todo!()
    }

    fn object(
		    &mut self,
		    vocabulary: &mut V,
		    interpretation: &mut VariableInterpretation,
		    value: ResourceInterpretation<VariableInterpretation, V>,
	    ) -> Result<Self::Object, linked_data::IntoQuadsError> {
        todo!()
    }

    fn graph(
		    &mut self,
		    vocabulary: &mut V,
		    interpretation: &mut VariableInterpretation,
		    value: ResourceInterpretation<VariableInterpretation, V>,
	    ) -> Result<Self::Subject, linked_data::IntoQuadsError> {
        todo!()
    }

    fn object_as_subject<'a>(
		    &self,
		    object: &'a Self::Object,
	    ) -> Result<&'a Self::Subject, linked_data::IntoQuadsError> {
        todo!()
    }

    fn subject_as_object<'a>(
		    &self,
		    subject: &'a Self::Subject,
	    ) -> Result<Self::ObjectRef<'a>, linked_data::IntoQuadsError>
	    where
		    <V>::Iri: 'a,
		    <V>::BlankId: 'a,
		    <V>::Literal: 'a,
		    <VariableInterpretation as Interpretation>::Resource: 'a {
        todo!()
    }

    fn object_as_ref<'a>(object: &'a Self::Object) -> Self::ObjectRef<'a>
	    where
		    <V>::Iri: 'a,
		    <V>::BlankId: 'a,
		    <V>::Literal: 'a,
		    <VariableInterpretation as Interpretation>::Resource: 'a {
        todo!()
    }

    fn cloned_object_ref<'a>(object_ref: Self::ObjectRef<'a>) -> Self::Object
	    where
		    <V>::Iri: 'a,
		    <V>::BlankId: 'a,
		    <V>::Literal: 'a,
		    <VariableInterpretation as Interpretation>::Resource: 'a {
        todo!()
    }
}



