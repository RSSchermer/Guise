use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Field, Ident, Lit, Meta};

use crate::error_log::ErrorLog;

pub fn expand_derive_attributes(input: &DeriveInput) -> Result<TokenStream, String> {
    if let Data::Struct(ref data) = input.data {
        let struct_name = &input.ident;
        let mod_path = quote!(guise);
        let mut log = ErrorLog::new();

        let mut fields: Vec<AttributeField> = Vec::new();

        'outer: for (i, field) in data.fields.iter().enumerate() {
            let field = AttributeField::from_ast(field, i, &mut log);

            for f in fields.iter() {
                if field.attribute_name == f.attribute_name {
                    log.log_error(format!(
                        "Fields `{}` and `{}` declare the same attribute name.",
                        &f.name, &field.name
                    ));

                    continue 'outer;
                }
            }

            fields.push(field);
        }

        let observed = fields.iter().map(|field| {
            let field_name = &field.name;
            let span = field.span;

            let attribute_name = if let Some(attribute_name) = field.attribute_name.as_ref() {
                quote!(#attribute_name)
            } else {
                quote!(#field_name)
            };

            quote_spanned!(span=> {
                #mod_path::name!(#attribute_name)
            })
        });

        let patterns = fields.iter().map(|field| {
            let field_name = &field.name;
            let field_ident = field
                .ident
                .clone()
                .map(|i| i.into_token_stream())
                .unwrap_or(field.position.into_token_stream());
            let span = field.span;

            let attribute_name = if let Some(attribute_name) = field.attribute_name.as_ref() {
                quote!(#attribute_name)
            } else {
                quote!(#field_name)
            };

            quote_spanned!(span=>
                #attribute_name => #mod_path::Attribute::update(&mut self.#field_ident, value)
            )
        });

        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let impl_block = quote! {
            #[automatically_derived]
            impl #impl_generics #mod_path::Attributes for #struct_name #ty_generics #where_clause {
                const OBSERVED: &'static [#mod_path::Name] = &[
                    #(#observed,)*
                ];

                fn update(
                    &mut self,
                    attribute_name: &#mod_path::Name,
                    value: Option<String>
                ) {
                    let as_str: &str = attribute_name.as_ref();

                    match as_str {
                        #(#patterns,)*
                        _ => ()
                    }
                }
            }
        };

        let suffix = struct_name.to_string().trim_start_matches("r#").to_owned();
        let dummy_const = Ident::new(
            &format!("_IMPL_ATTRIBUTES_FOR_{}", suffix),
            Span::call_site(),
        );

        let generated = quote! {
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const #dummy_const: () = {
                #[allow(unknown_lints)]
                #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
                #[allow(rust_2018_idioms)]

                #impl_block
            };
        };

        log.compile().map(|_| generated)
    } else {
        Err("`Attributes` can only be derived for a struct.".into())
    }
}

struct AttributeField {
    ident: Option<Ident>,
    position: usize,
    name: String,
    attribute_name: Option<String>,
    span: Span,
}

impl AttributeField {
    pub fn from_ast(ast: &Field, position: usize, log: &mut ErrorLog) -> Self {
        let field_name = ast
            .ident
            .clone()
            .map(|i| i.to_string())
            .unwrap_or(position.to_string());

        let name_attributes: Vec<&Attribute> = ast
            .attrs
            .iter()
            .filter(|a| a.path.is_ident("attribute_name"))
            .collect();

        if name_attributes.len() > 1 {
            log.log_error(format!(
                "Multiple #[attribute_name] attributes for field `{}`.",
                field_name
            ));
        }

        let mut attribute_name = None;

        if let Some(attr) = name_attributes.first() {
            match attr.parse_meta() {
                Ok(Meta::NameValue(meta)) => {
                    // if meta..nested.len() != 1 {
                    //     log.log_error(format!(
                    //         "Malformed #[attribute_name] attribute for field `{}`; expected a single \
                    //         string.",
                    //         field_name
                    //     ));
                    // } else {
                    //     let nested = meta.nested.first().unwrap();
                    //
                    //     if let NestedMeta::Lit(Lit::Str(lit)) = nested {
                    //         attribute_name = Some(lit.value());
                    //     } else {
                    //         log.log_error(format!(
                    //             "Malformed #[attribute_name] attribute for field `{}`; expected \
                    //             name to be a string integer literal.",
                    //             field_name
                    //         ));
                    //     }
                    // }

                    if let Lit::Str(lit) = &meta.lit {
                        attribute_name = Some(lit.value());
                    } else {
                        log.log_error(format!(
                            "Malformed #[attribute_name] attribute for field `{}`; expected \
                                name to be a string integer literal.",
                            field_name
                        ));
                    }
                }
                _ => {
                    log.log_error(format!(
                        "Malformed #[attribute_name] attribute for field `{}`.",
                        field_name
                    ));
                }
            }
        }

        AttributeField {
            ident: ast.ident.clone(),
            position,
            name: field_name,
            attribute_name,
            span: ast.span(),
        }
    }
}
