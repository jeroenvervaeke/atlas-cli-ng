use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, PathArguments, GenericArgument};

pub(crate) fn derive_try_from_map_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("TryFromMap can only be derived for structs with named fields"),
            }
        },
        _ => panic!("TryFromMap can only be derived for structs"),
    };

    let field_extractions = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_type = &field.ty;
        
        let extraction = if is_option_type(field_type) {
            let inner_type = get_type_from_option(field_type);
            if is_vec_type(inner_type) {
                quote! {
                    let #field_name = if let Some(values) = value.get(#field_name_str) {
                        if values.is_empty() {
                            None
                        } else {
                            Some(values.iter().map(|v| v.parse().map_err(|_| atlas_derive_core::TryFromMapError::ParseError {
                                field: #field_name_str.to_string(),
                                value: v.clone(),
                            })).collect::<Result<Vec<_>, _>>()?)
                        }
                    } else {
                        None
                    };
                }
            } else {
                quote! {
                    let #field_name = if let Some(values) = value.get(#field_name_str) {
                        if values.is_empty() {
                            None
                        } else {
                            Some(values[0].parse().map_err(|_| atlas_derive_core::TryFromMapError::ParseError {
                                field: #field_name_str.to_string(),
                                value: values[0].clone(),
                            })?)
                        }
                    } else {
                        None
                    };
                }
            }
        } else if is_vec_type(field_type) {
            quote! {
                let #field_name = value.get(#field_name_str)
                    .ok_or_else(|| atlas_derive_core::TryFromMapError::MissingField(#field_name_str.to_string()))?
                    .iter()
                    .map(|v| v.parse().map_err(|_| atlas_derive_core::TryFromMapError::ParseError {
                        field: #field_name_str.to_string(),
                        value: v.clone(),
                    }))
                    .collect::<Result<Vec<_>, _>>()?;
            }
        } else {
            quote! {
                let #field_name = value.get(#field_name_str)
                    .ok_or_else(|| atlas_derive_core::TryFromMapError::MissingField(#field_name_str.to_string()))?
                    .get(0)
                    .ok_or_else(|| atlas_derive_core::TryFromMapError::NoValuesInField(#field_name_str.to_string()))?
                    .parse()
                    .map_err(|_| atlas_derive_core::TryFromMapError::ParseError {
                        field: #field_name_str.to_string(),
                        value: value[#field_name_str][0].clone(),
                    })?;
            }
        };
        
        extraction
    });

    let field_names = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! { #field_name }
    });

    let expanded = quote! {
        impl TryFrom<std::collections::HashMap<String, Vec<String>>> for #name {
            type Error = atlas_derive_core::TryFromMapError;

            fn try_from(value: std::collections::HashMap<String, Vec<String>>) -> Result<Self, Self::Error> {
                #(#field_extractions)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            return segment.ident == "Option";
        }
    }
    false
}

fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            return segment.ident == "Vec";
        }
    }
    false
}

fn get_type_from_option(ty: &Type) -> &Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                    return inner_type;
                }
            }
        }
    }
    panic!("Not an Option type")
}
