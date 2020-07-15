use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Meta};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let iost = crate::root_path(&input);
    let name = input.ident.clone();

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#iost::Read));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (is_singleton, table_name) = input.attrs.iter().fold((false, None), |(a, b), attr| {
        match attr.parse_meta() {
            Ok(meta) => {
                let name = meta.path().get_ident().as_ref().expect("please add table name.").to_string();
                if name == "table_name" {
                    if b.is_some() {
                        panic!("only 1 table_name attribute allowed per struct");
                    }
                    match meta {
                        Meta::NameValue(meta) => {
                            let lit = meta.lit;
                            let s = Ident::new(format!("{}", quote!(#lit)).as_str().trim_matches('"'), Span::call_site());
                            (a, Some(s))
                        }
                        _ => {
                            panic!("invalid table_name attribute. must be in the form #[table_name = \"test\"]");
                        }
                    }
                } else if name == "singleton" {
                    if a {
                        panic!("only 1 singleton attribute allowed per struct");
                    }
                    (true, b)
                } else {
                    (a, b)
                }
            }
            Err(_) => (a, b),
        }
    });

    if table_name.is_none() {
        panic!("#[table_name] attribute must be used when deriving from Table");
    }

    let expanded = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut primary_key = None;
                let mut secondary_keys = Vec::new();
                for field in fields.named.iter() {
                    for attr in field.attrs.iter() {
                        let name = attr.parse_meta().unwrap();
                        let (is_primary, is_secondary) = name
                            .path()
                            .get_ident()
                            .map(|n| {
                                let n = n.to_string();
                                (n == "primary", n == "secondary")
                            })
                            .unwrap_or_else(|| (false, false));

                        if is_singleton {
                            if is_primary {
                                panic!("primary key attribute not allowed with singletons");
                            } else if is_secondary {
                                panic!("secondary key attribute not allowed with singletons");
                            }
                        }

                        if is_primary {
                            if primary_key.is_none() {
                                primary_key = field.ident.clone();
                            } else {
                                panic!("only 1 primary key allowed");
                            }
                        }

                        if is_secondary {
                            secondary_keys.push((field.ident.clone(), field.ty.clone()));
                        }
                    }
                }

                if is_singleton {
                    quote! {
                        #[automatically_derived]
                        impl #impl_generics #iost::Table for #name #ty_generics #where_clause {
                            const NAME: u64 = #iost::n!(#table_name);

                            type Row = Self;

                            #[inline]
                            fn primary_key(_row: &Self::Row) -> u64 {
                                Self::NAME
                            }
                        }

                        #[automatically_derived]
                        impl #impl_generics #name #ty_generics #where_clause {
                            #[inline]
                            pub fn singleton<C, S>(code: C, scope: S) -> #iost::SingletonIndex<Self>
                            where
                                C: Into<AccountName>,
                                S: Into<ScopeName>,
                            {
                                #iost::SingletonIndex::new(code, scope)
                            }
                        }
                    }
                } else {
                    if primary_key.is_none() {
                        panic!("no primary key found");
                    }
                    if secondary_keys.len() > 16 {
                        panic!("up to 16 secondary keys are allowed");
                    }

                    let mut secondary_keys_expanded = quote!();
                    let mut secondary_keys_constructors = quote!();
                    for i in 0..16 {
                        match secondary_keys.get(i) {
                            Some((ident, ty)) => {
                                secondary_keys_expanded = quote! {
                                    #secondary_keys_expanded
                                    Some(#iost::SecondaryKey::from(row.#ident)),
                                };
                                let ident = Ident::new(
                                    format!("by_{}", quote!(#ident)).as_str(),
                                    Span::call_site(),
                                );
                                secondary_keys_constructors = quote! {
                                    #secondary_keys_constructors

                                    #[inline]
                                    pub fn #ident<C, S>(code: C, scope: S) -> #iost::SecondaryTableIndex<#ty, Self>
                                    where
                                        C: Into<#iost::AccountName>,
                                        S: Into<#iost::ScopeName>,
                                    {
                                        #iost::SecondaryTableIndex::new(code, scope, #iost::n!(#table_name), #i)
                                    }
                                };
                            }
                            None => {
                                secondary_keys_expanded = quote! {
                                    #secondary_keys_expanded
                                    None,
                                };
                            }
                        };
                    }

                    quote! {
                        #[automatically_derived]
                        impl #impl_generics #iost::Table for #name #ty_generics #where_clause {
                            const NAME: u64 = #iost::n!(#table_name);

                            type Row = Self;

                            #[inline]
                            fn primary_key(row: &Self::Row) -> u64 {
                                row.#primary_key.into()
                            }

                            #[inline]
                            fn secondary_keys(row: &Self::Row) -> #iost::SecondaryKeys {
                                SecondaryKeys::from([
                                    #secondary_keys_expanded
                                ])
                            }
                        }

                        #[automatically_derived]
                        impl #impl_generics #name #ty_generics #where_clause {
                            #secondary_keys_constructors
                        }
                    }
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    TokenStream::from(expanded)
}
