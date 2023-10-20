#[macro_export]
macro_rules! json_literal {
	($ty:ty) => {
		impl<V: $crate::rdf_types::Vocabulary, I: $crate::rdf_types::Interpretation> $crate::LinkedDataResource<I, V> for $ty {
			fn interpretation(
				&self,
				_vocabulary: &mut V,
				_interpretation: &mut I,
			) -> $crate::ResourceInterpretation<I, V> {
				use $crate::{rdf_types::Term, CowRdfTerm, RdfLiteral, ResourceInterpretation};

				let mut value = $crate::json_syntax::to_value(self).unwrap();
				value.canonicalize();

				ResourceInterpretation::Uninterpreted(Some(CowRdfTerm::Owned(Term::Literal(
					RdfLiteral::Json(value),
				))))
			}
		}

		impl<V: $crate::rdf_types::Vocabulary, I: $crate::rdf_types::Interpretation> $crate::LinkedDataPredicateObjects<I, V> for $ty {
			fn visit_objects<S>(&self, mut visitor: S) -> Result<S::Ok, S::Error>
			where
				S: $crate::PredicateObjectsVisitor<I, V>,
			{
				visitor.object(self)?;
				visitor.end()
			}
		}

		impl<V: $crate::rdf_types::Vocabulary, I: $crate::rdf_types::Interpretation> $crate::LinkedDataDeserializePredicateObjects<I, V> for $ty
		where
			V: $crate::rdf_types::Vocabulary<Type = $crate::rdf_types::literal::Type<<V as $crate::rdf_types::IriVocabulary>::Iri, <V as $crate::rdf_types::LanguageTagVocabulary>::LanguageTag>>,
			V::Value: AsRef<str>,
			I: $crate::rdf_types::ReverseLiteralInterpretation<Literal = V::Literal>
		{
			fn deserialize_objects<'a, D>(
				vocabulary: &V,
				interpretation: &I,
				dataset: &D,
				graph: &D::Graph,
				objects: impl IntoIterator<Item = &'a I::Resource>,
			) -> Result<Self, $crate::FromLinkedDataError>
			where
				I::Resource: 'a,
				D: $crate::grdf::Dataset<
					Subject = I::Resource,
					Predicate = I::Resource,
					Object = I::Resource,
					GraphLabel = I::Resource,
				>
			{
				let mut objects = objects.into_iter();
				match objects.next() {
					Some(object) => {
						if objects.next().is_none() {
							<Self as $crate::LinkedDataDeserializeSubject<I, V>>::deserialize_subject(vocabulary, interpretation, dataset, graph, object)
						} else {
							Err($crate::FromLinkedDataError::TooManyValues)
						}
					}
					None => {
						Err($crate::FromLinkedDataError::MissingRequiredValue)
					}
				}
			}
		}

		impl<V: $crate::rdf_types::Vocabulary, I: $crate::rdf_types::Interpretation> $crate::LinkedDataSubject<I, V> for $ty {
			fn visit_subject<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
			where
				S: $crate::SubjectVisitor<I, V>,
			{
				visitor.end()
			}
		}

		impl<V: $crate::rdf_types::Vocabulary, I: $crate::rdf_types::Interpretation> $crate::LinkedDataDeserializeSubject<I, V> for $ty
		where
			V: $crate::rdf_types::Vocabulary<Type = $crate::rdf_types::literal::Type<<V as $crate::rdf_types::IriVocabulary>::Iri, <V as $crate::rdf_types::LanguageTagVocabulary>::LanguageTag>>,
			V::Value: AsRef<str>,
			I: $crate::rdf_types::ReverseLiteralInterpretation<Literal = V::Literal>
		{
			fn deserialize_subject<D>(
				vocabulary: &V,
				interpretation: &I,
				_dataset: &D,
				_graph: &D::Graph,
				resource: &I::Resource,
			) -> Result<Self, $crate::FromLinkedDataError>
			where
				D: $crate::grdf::Dataset<
					Subject = I::Resource,
					Predicate = I::Resource,
					Object = I::Resource,
					GraphLabel = I::Resource,
				>
			{
				use $crate::rdf_types::literal;

				let mut literal_ty = None;
				for l in interpretation.literals_of(resource) {
					let literal = vocabulary.literal(l).unwrap();
					match literal.type_() {
						literal::Type::Any(ty) => {
							let ty_iri = vocabulary.iri(ty).unwrap();
							if ty_iri == $crate::rdf_types::RDF_JSON {
								use $crate::json_syntax::Parse;
								let json = $crate::json_syntax::Value::parse_str(literal.value().as_ref(), |_| ())
									.map_err(|_| $crate::FromLinkedDataError::InvalidLiteral)?;

								return $crate::json_syntax::from_meta_value(json).map_err(|_| $crate::FromLinkedDataError::InvalidLiteral)
							} else {
								literal_ty = Some(ty_iri)
							}
						}
						literal::Type::LangString(_) => {
							literal_ty = Some($crate::rdf_types::RDF_LANG_STRING)
						}
					}
				}

				match literal_ty {
					Some(ty) => {
						Err($crate::FromLinkedDataError::LiteralTypeMismatch {
							property: None,
							expected: Some($crate::rdf_types::RDF_JSON.to_owned()),
							found: ty.to_owned()
						})
					}
					None => {
						Err($crate::FromLinkedDataError::ExpectedLiteral)
					}
				}
			}
		}
	};
}

#[cfg(test)]
mod test {
	#[derive(Debug, serde::Serialize, serde::Deserialize)]
	struct Test {
		field: String,
	}

	json_literal!(Test);
}
