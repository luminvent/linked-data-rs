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
//! let quads = linked_data::to_quads(rdf_types::generator::Blank::new(), &value).expect("RDF serialization failed");
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
use iref::Iri;
#[cfg(feature = "derive")]
pub use linked_data_derive::{Deserialize, Serialize};
use rdf_types::{Interpretation, Vocabulary};

#[doc(hidden)]
pub use iref;

#[doc(hidden)]
pub use rdf_types;

#[doc(hidden)]
pub use xsd_types;

#[doc(hidden)]
pub use json_syntax;

#[doc(hidden)]
pub use grdf;

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
mod subject;

pub use anonymous::*;
pub use graph::*;
pub use predicate::*;
pub use quads::{
	to_interpreted_graph_quads, to_interpreted_quads, to_interpreted_subject_quads,
	to_lexical_quads, to_lexical_quads_with, to_quads, to_quads_with, IntoQuadsError,
};
pub use rdf::*;
pub use reference::*;
pub use resource::*;
pub use subject::*;

#[derive(Debug, thiserror::Error)]
pub enum FromLinkedDataError {
	/// Resource has no literal interpretation.
	#[error("expected literal")]
	ExpectedLiteral,

	/// Resource has literal interpretations, but none of the expected type.
	#[error("literal type mismatch")]
	LiteralTypeMismatch,

	/// Resource has a literal interpretation of the correct type, but the
	/// lexical value could not be successfully parsed.
	#[error("invalid literal")]
	InvalidLiteral,

	/// Missing required value.
	#[error("missing required value")]
	MissingRequiredValue,

	/// Too many values.
	#[error("too many values")]
	TooManyValues,

	/// Generic error for invalid subjects.
	#[error("invalid subject")]
	InvalidSubject,
}

/// Linked-Data type.
///
/// A Linked-Data type represents an RDF dataset which can be visited using the
/// [`visit`](Self::visit) method.
pub trait LinkedData<V: Vocabulary = (), I: Interpretation = ()> {
	/// Visit the RDF dataset represented by this type.
	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<V, I>;
}

impl<'a, V: Vocabulary, I: Interpretation, T: ?Sized + LinkedData<V, I>> LinkedData<V, I>
	for &'a T
{
	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<V, I>,
	{
		T::visit(self, visitor)
	}
}

impl<V: Vocabulary, I: Interpretation, T: ?Sized + LinkedData<V, I>> LinkedData<V, I> for Box<T> {
	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<V, I>,
	{
		T::visit(self, visitor)
	}
}

impl<V: Vocabulary, I: Interpretation> LinkedData<V, I> for Iri {
	fn visit<S>(&self, visitor: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<V, I>,
	{
		visitor.end()
	}
}

/// RDF dataset visitor.
pub trait Visitor<V: Vocabulary, I: Interpretation> {
	/// Type of the value returned by the visitor when the dataset has been
	/// entirely visited.
	type Ok;

	/// Error type.
	type Error;

	/// Visits the default graph of the dataset.
	fn default_graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataGraph<V, I>;

	/// Visits a named graph of the dataset.
	fn named_graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataResource<V, I> + LinkedDataGraph<V, I>;

	/// Ends the dataset visit.
	fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Any mutable reference to a visitor is itself a visitor.
impl<'s, V: Vocabulary, I: Interpretation, S: Visitor<V, I>> Visitor<V, I> for &'s mut S {
	type Ok = ();
	type Error = S::Error;

	fn default_graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataGraph<V, I>,
	{
		S::default_graph(self, value)
	}

	fn named_graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataResource<V, I> + LinkedDataGraph<V, I>,
	{
		S::named_graph(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}

pub trait LinkedDataDeserialize<V: Vocabulary = (), I: Interpretation = ()>: Sized {
	fn deserialize_dataset(
		vocabulary: &V,
		interpretation: &I,
		dataset: &impl grdf::Dataset<
			Subject = I::Resource,
			Predicate = I::Resource,
			Object = I::Resource,
			GraphLabel = I::Resource,
		>,
	) -> Result<Self, FromLinkedDataError>;
}
