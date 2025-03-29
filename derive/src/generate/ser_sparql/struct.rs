
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;
use crate::generate::{extend_generics, read_field_attributes, InterpretationBounds, TypeAttributes};

use super::{variant_compound_fields, Error};

pub fn generate(
	attrs: &TypeAttributes,
	ident: Ident,
	generics: syn::Generics,
	s: syn::DataStruct,
) -> Result<TokenStream, Error> {
	let mut construct_content = String::new();
	let mut where_content = String::new();

	for field in &s.fields {
		let field_attrs = read_field_attributes(field.attrs.clone())?;

		if !field_attrs.ignore {

			if let Some(compact_iri) = field_attrs.iri {
				let iri = compact_iri.expand(&attrs.prefixes)?.into_string();

				let object = field.ident.clone().unwrap().to_string();

				if field_attrs.flatten {

				} else {
					construct_content.push_str(&format!("?{} <{}> ?{}_value . ", object, iri, object));
					where_content.push_str(&format!("?{} <{}> ?{}_value .", object, iri, object));
				}
			} else {
				panic!()
			}
		}
	}

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

	let bounds: Vec<syn::WherePredicate> = fields.visit.bounds;
	let vocabulary_bounds = fields.visit.vocabulary_bounds;

	let ld_generics = extend_generics(
		&generics,
		vocabulary_bounds,
		InterpretationBounds::default(),
		bounds,
	);
	let (_, ty_generics, _) = generics.split_for_impl();
	let (impl_generics, _, where_clause) = ld_generics.split_for_impl();

	let list_construct = format!("CONSTRUCT {{ {construct_content} }}");
	let list_where = format!("WHERE {{ {where_content} }}");
	let list_query = format!("{list_construct} {list_where}");

	let list_query = LitStr::new(&list_query, Span::call_site());

	Ok(quote! {
		impl #impl_generics ::linked_data::SparqlQueryGenerator<I_, V_> for #ident #ty_generics #where_clause {
			fn generate_list_query() -> String {
				#list_query.to_string()
			}

			fn generate_get_query(id: &str) -> String {
				"".to_string()
			}
		}
	})
}
