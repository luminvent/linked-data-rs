use rdf_types::{
	Id, Interpretation, Term, Vocabulary,
	vocabulary::{IriVocabularyMut, LiteralVocabularyMut},
};

use crate::{
	CowRdfTerm, LinkedDataPredicateObjects, LinkedDataResource, LinkedDataSubject,
	PredicateObjectsVisitor, RdfLiteral, RdfLiteralRef, ResourceInterpretation,
};

macro_rules! datatype {
	($ty:ty, $variant:ident) => {
		impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation>
			LinkedDataResource<I, V> for $ty
		{
			fn interpretation(
				&self,
				_vocabulary: &mut V,
				_interpretation: &mut I,
			) -> ResourceInterpretation<'_, I, V> {
				ResourceInterpretation::Uninterpreted(Some(CowRdfTerm::Owned(Term::Literal(
					RdfLiteral::Xsd(xsd_types::Value::$variant(self.clone().into())),
				))))
			}
		}

		impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation>
			LinkedDataSubject<I, V> for $ty
		{
			fn visit_subject<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
			where
				S: crate::SubjectVisitor<I, V>,
			{
				visitor.end()
			}
		}

		impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation>
			LinkedDataPredicateObjects<I, V> for $ty
		{
			fn visit_objects<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
			where
				S: PredicateObjectsVisitor<I, V>,
			{
				visitor.object(self)?;
				visitor.end()
			}
		}
	};
}

macro_rules! unsized_datatype {
	($($ty:ty : $variant:ident),*) => {
		$(
			impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation> LinkedDataResource<I, V> for $ty {
				fn interpretation(
					&self,
					_vocabulary: &mut V,
					_interpretation: &mut I,
				) -> ResourceInterpretation<'_, I, V> {
					ResourceInterpretation::Uninterpreted(Some(CowRdfTerm::Borrowed(Term::Literal(RdfLiteralRef::Xsd(
						xsd_types::ValueRef::$variant(self)
					)))))
				}
			}

			impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation> LinkedDataSubject<I, V> for $ty {
				fn visit_subject<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
				where
					S: crate::SubjectVisitor<I, V>
				{
					visitor.end()
				}
			}

			impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation> LinkedDataPredicateObjects<I, V> for $ty {
				fn visit_objects<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
				where
					S: PredicateObjectsVisitor<I, V>,
				{
					visitor.object(self)?;
					visitor.end()
				}
			}
		)*
	};
}

datatype!(bool, Boolean);
datatype!(u8, UnsignedByte);
datatype!(u16, UnsignedShort);
datatype!(u32, UnsignedInt);
datatype!(u64, UnsignedLong);
datatype!(i8, Byte);
datatype!(i16, Short);
datatype!(i32, Int);
datatype!(i64, Long);
datatype!(f32, Float);
datatype!(f64, Double);
datatype!(String, String);
datatype!(xsd_types::DateTime, DateTime);

unsized_datatype! {
	str: String
}

impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation>
	LinkedDataResource<I, V> for xsd_types::AnyUriBuf
{
	fn interpretation(
		&self,
		_vocabulary: &mut V,
		_interpretation: &mut I,
	) -> ResourceInterpretation<'_, I, V> {
		ResourceInterpretation::Uninterpreted(Some(CowRdfTerm::Owned(Term::Literal(
			RdfLiteral::Xsd(xsd_types::Value::AnyUri(self.clone())),
		))))
	}

	fn reference_interpretation(
		&self,
		vocabulary: &mut V,
		_interpretation: &mut I,
	) -> ResourceInterpretation<'_, I, V> {
		ResourceInterpretation::Uninterpreted(Some(CowRdfTerm::Owned(Term::Id(Id::Iri(
			vocabulary.insert(self.as_iri()),
		)))))
	}
}

impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation>
	LinkedDataSubject<I, V> for xsd_types::AnyUriBuf
{
	fn visit_subject<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: crate::SubjectVisitor<I, V>,
	{
		visitor.end()
	}
}

impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation>
	LinkedDataPredicateObjects<I, V> for xsd_types::AnyUriBuf
{
	fn visit_objects<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
	where
		S: PredicateObjectsVisitor<I, V>,
	{
		visitor.object(self)?;
		visitor.end()
	}
}
