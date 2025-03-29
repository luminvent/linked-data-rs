//! This library provides primitive traits to serialize and deserialize
//! Linked-Data types. It is shipped with derive macros (using the `derive`
//! feature) that can automatically implement those primitives for you.
//!
//! # Example
//!
//! ```
//! use iref::IriBuf;
//! use static_iref::iri;
//!
//! #[derive(linked_data::Serialize, linked_data::Deserialize)]
//! #[ld(prefix("ex" = "http://example.org/"))]
//! struct Foo {
//!   #[ld(id)]
//!   id: IriBuf,
//!
//!   #[ld("ex:name")]
//!   name: String,
//!
//!   #[ld("ex:email")]
//!   email: String
//! }
//!
//! let value = Foo {
//!   id: iri!("http://example.org/JohnSmith").to_owned(),
//!   name: "John Smith".to_owned(),
//!   email: "john.smith@example.org".to_owned()
//! };
//!
//! let quads = linked_data::to_quads(rdf_types::generator::Blank::new(), &value)
//!   .expect("RDF serialization failed");
//!
//! for quad in quads {
//!   use rdf_types::RdfDisplay;
//!   println!("{} .", quad.rdf_display())
//! }
//! ```
//!
//! This should print the following:
//! ```text
//! <http://example.org/JohnSmith> <http://example.org/name> "John Smith" .
//! <http://example.org/JohnSmith> <http://example.org/email> "john.smith@example.org" .
//! ```
use educe::Educe;
use iref::{Iri, IriBuf};
#[cfg(feature = "derive")]
pub use linked_data_derive::{Deserialize, Serialize, SparqlSerialize};
use rdf_types::{
	dataset::{PatternMatchingDataset, TraversableDataset},
	interpretation::ReverseIriInterpretation,
	vocabulary::IriVocabulary,
	Interpretation, Vocabulary,
};

#[doc(hidden)]
pub use iref;

#[doc(hidden)]
pub use rdf_types;

#[doc(hidden)]
pub use xsd_types;

#[doc(hidden)]
pub use json_syntax;

mod anonymous;
mod datatypes;
mod graph;
mod r#impl;
mod macros;
mod predicate;
mod quads;
mod rdf;
mod reference;
mod resource;
mod sparql;
mod subject;

pub use anonymous::*;
pub use graph::*;
pub use predicate::*;
pub use quads::{
	to_interpreted_graph_quads, to_interpreted_quads, to_interpreted_subject_quads,
	to_lexical_quads, to_lexical_quads_with, to_lexical_subject_quads,
	to_lexical_subject_quads_with, to_quads, to_quads_with, IntoQuadsError,
};
pub use rdf::*;
pub use reference::*;
pub use resource::*;
pub use sparql::*;
pub use subject::*;

#[derive(Debug, thiserror::Error)]
pub enum FromLinkedDataError {
	/// Resource has no IRI representation.
	#[error("expected IRI")]
	ExpectedIri(ContextIris),

	#[error("unsupported IRI `{found}`")]
	UnsupportedIri {
		/// Error context.
		context: ContextIris,

		/// Unsupported IRI.
		found: IriBuf,

		/// Optional hint listing the supported IRIs.
		supported: Option<Vec<IriBuf>>,
	},

	/// Resource has no literal representation.
	#[error("expected literal")]
	ExpectedLiteral(ContextIris),

	/// Resource has literal representations, but none of the expected type.
	#[error("literal type mismatch")]
	LiteralTypeMismatch {
		context: ContextIris,
		expected: Option<IriBuf>,
		found: IriBuf,
	},

	/// Resource has a literal representation of the correct type, but the
	/// lexical value could not be successfully parsed.
	#[error("invalid literal")]
	InvalidLiteral(ContextIris),

	/// Missing required value.
	#[error("missing required value")]
	MissingRequiredValue(ContextIris),

	/// Too many values.
	#[error("too many values")]
	TooManyValues(ContextIris),

	/// Generic error for invalid subjects.
	#[error("invalid subject")]
	InvalidSubject {
		context: ContextIris,
		subject: Option<IriBuf>,
	},
}

impl FromLinkedDataError {
	pub fn context(&self) -> &ContextIris {
		match self {
			Self::ExpectedIri(c) => c,
			Self::UnsupportedIri { context, .. } => context,
			Self::ExpectedLiteral(c) => c,
			Self::LiteralTypeMismatch { context, .. } => context,
			Self::InvalidLiteral(c) => c,
			Self::MissingRequiredValue(c) => c,
			Self::TooManyValues(c) => c,
			Self::InvalidSubject { context, .. } => context,
		}
	}
}

/// Linked-Data type.
///
/// A Linked-Data type represents an RDF dataset which can be visited using the
/// [`visit`](Self::visit) method.
pub trait LinkedData<I: Interpretation = (), V: Vocabulary = ()> {
	/// Visit the RDF dataset represented by this type.
	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<I, V>;
}

impl<'a, I: Interpretation, V: Vocabulary, T: ?Sized + LinkedData<I, V>> LinkedData<I, V>
	for &'a T
{
	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<I, V>,
	{
		T::visit(self, visitor)
	}
}

impl<I: Interpretation, V: Vocabulary, T: ?Sized + LinkedData<I, V>> LinkedData<I, V> for Box<T> {
	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<I, V>,
	{
		T::visit(self, visitor)
	}
}

impl<I: Interpretation, V: Vocabulary> LinkedData<I, V> for Iri {
	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<I, V>,
	{
		visitor.end()
	}
}

/// RDF dataset visitor.
pub trait Visitor<I: Interpretation = (), V: Vocabulary = ()> {
	/// Type of the value returned by the visitor when the dataset has been
	/// entirely visited.
	type Ok;

	/// Error type.
	type Error;

	/// Visits the default graph of the dataset.
	fn default_graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataGraph<I, V>;

	/// Visits a named graph of the dataset.
	fn named_graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataResource<I, V> + LinkedDataGraph<I, V>;

	/// Ends the dataset visit.
	fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Any mutable reference to a visitor is itself a visitor.
impl<'s, I: Interpretation, V: Vocabulary, S: Visitor<I, V>> Visitor<I, V> for &'s mut S {
	type Ok = ();
	type Error = S::Error;

	fn default_graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataGraph<I, V>,
	{
		S::default_graph(self, value)
	}

	fn named_graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataResource<I, V> + LinkedDataGraph<I, V>,
	{
		S::named_graph(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}

#[derive(Educe)]
#[educe(Debug(bound = "I::Resource: core::fmt::Debug"), Clone, Copy)]
pub enum ResourceOrIriRef<'a, I: Interpretation> {
	Resource(&'a I::Resource),
	Iri(&'a Iri),
	Anonymous,
}

impl<'a, I: Interpretation> ResourceOrIriRef<'a, I> {
	pub fn into_iri<V>(self, vocabulary: &V, interpretation: &I) -> Option<IriBuf>
	where
		V: IriVocabulary,
		I: ReverseIriInterpretation<Iri = V::Iri>,
	{
		match self {
			Self::Resource(r) => interpretation
				.iris_of(r)
				.next()
				.map(|i| vocabulary.iri(i).unwrap().to_owned()),
			Self::Iri(i) => Some(i.to_owned()),
			Self::Anonymous => None,
		}
	}
}

#[derive(Educe)]
#[educe(Debug(bound = "I::Resource: core::fmt::Debug"), Clone, Copy)]
pub enum Context<'a, I: Interpretation> {
	Subject,
	Predicate {
		subject: ResourceOrIriRef<'a, I>,
	},
	Object {
		subject: ResourceOrIriRef<'a, I>,
		predicate: ResourceOrIriRef<'a, I>,
	},
}

impl<'a, I: Interpretation> Context<'a, I> {
	pub fn with_subject(self, subject: &'a I::Resource) -> Self {
		Self::Predicate {
			subject: ResourceOrIriRef::Resource(subject),
		}
	}

	pub fn with_predicate(self, predicate: &'a I::Resource) -> Self {
		match self {
			Self::Predicate { subject } => Self::Object {
				subject,
				predicate: ResourceOrIriRef::Resource(predicate),
			},
			_ => Self::Subject,
		}
	}

	pub fn with_predicate_iri(self, predicate: &'a Iri) -> Self {
		match self {
			Self::Predicate { subject } => Self::Object {
				subject,
				predicate: ResourceOrIriRef::Iri(predicate),
			},
			_ => Self::Subject,
		}
	}

	pub fn with_anonymous_predicate(self) -> Self {
		match self {
			Self::Predicate { subject } => Self::Object {
				subject,
				predicate: ResourceOrIriRef::Anonymous,
			},
			_ => Self::Subject,
		}
	}

	pub fn into_iris<V>(self, vocabulary: &V, interpretation: &I) -> ContextIris
	where
		V: IriVocabulary,
		I: ReverseIriInterpretation<Iri = V::Iri>,
	{
		match self {
			Self::Subject => ContextIris::Subject,
			Self::Predicate { subject } => ContextIris::Predicate {
				subject: subject.into_iri(vocabulary, interpretation),
			},
			Self::Object { subject, predicate } => ContextIris::Object {
				subject: subject.into_iri(vocabulary, interpretation),
				predicate: predicate.into_iri(vocabulary, interpretation),
			},
		}
	}
}

#[derive(Debug, Clone)]
pub enum ContextIris {
	Subject,
	Predicate {
		subject: Option<IriBuf>,
	},
	Object {
		subject: Option<IriBuf>,
		predicate: Option<IriBuf>,
	},
}

impl<'a, I: Interpretation> Default for Context<'a, I> {
	fn default() -> Self {
		Self::Subject
	}
}

pub trait LinkedDataDeserialize<V: Vocabulary = (), I: Interpretation = ()>: Sized {
	fn deserialize_dataset_in(
		vocabulary: &V,
		interpretation: &I,
		dataset: &(impl TraversableDataset<Resource = I::Resource> + PatternMatchingDataset),
		context: Context<I>,
	) -> Result<Self, FromLinkedDataError>;

	fn deserialize_dataset(
		vocabulary: &V,
		interpretation: &I,
		dataset: &(impl TraversableDataset<Resource = I::Resource> + PatternMatchingDataset),
	) -> Result<Self, FromLinkedDataError> {
		Self::deserialize_dataset_in(vocabulary, interpretation, dataset, Context::default())
	}
}
