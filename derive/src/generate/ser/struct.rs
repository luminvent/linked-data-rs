use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::generate::{InterpretationBounds, RDF_TYPE, TypeAttributes, extend_generics};

use super::{Error, variant_compound_fields};

pub fn generate(
	attrs: &TypeAttributes,
	ident: Ident,
	generics: syn::Generics,
	s: syn::DataStruct,
) -> Result<TokenStream, Error> {
	let fields = variant_compound_fields(
		attrs,
		s.fields,
		|f| quote!(self.#f),
		|i| {
			let index = syn::Index {
				index: i,
				span: Span::call_site(),
			};

			quote!(self.#index)
		},
		|t| quote!(&#t),
	)?;

	let mut bounds: Vec<syn::WherePredicate> = fields.visit.bounds;

	let visit_type = attrs
		.type_
		.as_ref()
		.map(|ty| {
			let iri = ty.expand(&attrs.prefixes)?.into_string();
			let rdf_type = RDF_TYPE.as_str();

			Ok(quote! {
				visitor.predicate(
					::linked_data_next::iref::Iri::new(#rdf_type).unwrap(),
					::linked_data_next::iref::Iri::new(#iri).unwrap()
				)?;
			})
		})
		.transpose()?;

	let visit = fields.visit.body;
	let vocabulary_bounds = fields.visit.vocabulary_bounds;

	let term = match fields.id_field {
		Some((field_access, ty)) => {
			bounds.push(
				syn::parse2(quote! {
					#ty: ::linked_data_next::LinkedDataResource<I_, V_>
				})
				.unwrap(),
			);

			quote! {
				#field_access.interpretation(vocabulary, interpretation)
			}
		}
		None => quote! {
			::linked_data_next::ResourceInterpretation::Uninterpreted(None)
		},
	};

	let ld_generics = extend_generics(
		&generics,
		vocabulary_bounds,
		InterpretationBounds::default(),
		bounds,
	);
	let (_, ty_generics, _) = generics.split_for_impl();
	let (impl_generics, _, where_clause) = ld_generics.split_for_impl();

	Ok(quote! {
		impl #impl_generics ::linked_data_next::LinkedDataResource<I_, V_> for #ident #ty_generics #where_clause {
			fn interpretation(
				&self,
				vocabulary: &mut V_,
				interpretation: &mut I_
			) -> ::linked_data_next::ResourceInterpretation<I_, V_> {
				#term
			}
		}

		impl #impl_generics ::linked_data_next::LinkedDataSubject<I_, V_> for #ident #ty_generics #where_clause {
			fn visit_subject<S_>(&self, mut visitor: S_) -> Result<S_::Ok, S_::Error>
			where
				S_: ::linked_data_next::SubjectVisitor<I_, V_>
			{
				#visit_type
				#visit
			}
		}

		impl #impl_generics ::linked_data_next::LinkedDataPredicateObjects<I_, V_> for #ident #ty_generics #where_clause {
			fn visit_objects<S_>(&self, mut visitor: S_) -> Result<S_::Ok, S_::Error>
			where
				S_: ::linked_data_next::PredicateObjectsVisitor<I_, V_>
			{
				visitor.object(self)?;
				visitor.end()
			}
		}

		impl #impl_generics ::linked_data_next::LinkedDataGraph<I_, V_> for #ident #ty_generics #where_clause {
			fn visit_graph<S_>(&self, mut visitor: S_) -> Result<S_::Ok, S_::Error>
			where
				S_: ::linked_data_next::GraphVisitor<I_, V_>
			{
				visitor.subject(self)?;
				visitor.end()
			}
		}

		impl #impl_generics ::linked_data_next::LinkedData<I_, V_> for #ident #ty_generics #where_clause {
			fn visit<S_>(&self, mut visitor: S_) -> Result<S_::Ok, S_::Error>
			where
				S_: ::linked_data_next::Visitor<I_, V_>
			{
				visitor.default_graph(self)?;
				visitor.end()
			}
		}
	})
}
