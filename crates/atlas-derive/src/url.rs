use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, Fields, GenericArgument, Lit, PathArguments, Type, parse_macro_input,
};

/// Checks if a type is an `Option<T>`.
fn is_option_type(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.segments.last()
        .map_or(false, |seg| seg.ident == "Option"))
}

/// Checks if a type is a `Vec<T>`.
fn is_vec_type(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.segments.last()
        .map_or(false, |seg| seg.ident == "Vec"))
}

/// Extracts the inner type from an `Option<T>` or `Vec<T>`.
fn get_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        type_path.path.segments.last().and_then(|segment| {
            match &segment.arguments { PathArguments::AngleBracketed(args) => {
                args.args.first().and_then(|arg| {
                    if let GenericArgument::Type(inner_ty) = arg {
                        Some(inner_ty)
                    } else {
                        None
                    }
                })
            } _ => {
                None
            }}
        })
    } else {
        None
    }
}

/// Checks if a field name is used as a path parameter in the URL pattern.
fn is_path_param(field_name: &str, url_pattern: &str) -> bool {
    url_pattern.contains(&format!("{{{}}}", field_name))
}

/// Implements the `AsUrl` derive macro which generates URL handling code.
///
/// This macro generates an implementation of the `AsUrl` trait for structs,
/// allowing them to be converted into URLs with proper handling of:
/// - Path parameters (using {param} syntax in the URL pattern)
/// - Query parameters (fields not used in the path)
/// - Optional parameters (Option<T> fields)
/// - Vector parameters (Vec<T> fields)
///
/// # Example
/// ```ignore
/// #[derive(AtlasURL)]
/// #[url("/api/v1/users/{id}")]
/// struct User {
///     id: u32,
///     name: Option<String>,
///     tags: Vec<String>,
/// }
/// ```
pub(crate) fn derive_atlas_url_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract URL pattern from the #[url("...")] attribute
    let url_pattern = input.attrs
        .iter()
        .find(|attr| attr.path().is_ident("url"))
        .and_then(|attr| {
            let expr = attr.parse_args::<Expr>().ok()?;
            match expr {
                Expr::Lit(expr_lit) => {
                    match expr_lit.lit {
                        Lit::Str(s) => Some(s.value()),
                        _ => None
                    }
                },
                _ => None
            }
        })
        .unwrap_or_else(|| {
            panic!("AtlasURL requires a URL pattern with #[url(\"/path/to/resource\")]")
        });

    let name = &input.ident;

    // Extract named fields from the struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("AtlasURL only supports named fields"),
        },
        _ => panic!("AtlasURL only supports structs"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // Generate code for path segments
    let segments: Vec<_> = url_pattern
        .split('{')
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                return quote! { url.push_str(#s); };
            }

            let (field, rest) = s.split_once('}')
                .unwrap_or_else(|| panic!("Invalid URL pattern: missing closing brace"));
            
            let field = field.trim();
            let field_ident = syn::Ident::new(field, proc_macro2::Span::call_site());

            // Find the field type and generate appropriate serialization code
            let field_type = fields
                .iter()
                .find(|f| f.ident.as_ref().unwrap() == &field_ident)
                .map(|f| &f.ty);

            match field_type {
                Some(ty) if is_vec_type(ty) => quote! {
                    url.push_str(&self.#field_ident.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(","));
                    url.push_str(#rest);
                },
                Some(ty) if is_option_type(ty) => quote! {
                    if let Some(value) = self.#field_ident {
                        url.push_str(&value.to_string());
                    }
                    url.push_str(#rest);
                },
                _ => quote! {
                    url.push_str(&self.#field_ident.to_string());
                    url.push_str(#rest);
                },
            }
        })
        .collect();

    // Check for query parameters
    let has_query_params = field_names
        .iter()
        .zip(field_types.iter())
        .any(|(name, ty)| !is_path_param(&name.to_string(), &url_pattern) && !is_option_type(ty));

    let has_optional_params = field_names
        .iter()
        .zip(field_types.iter())
        .any(|(name, ty)| {
            !is_path_param(&name.to_string(), &url_pattern)
                && (is_option_type(ty) || is_vec_type(ty))
        });

    // Generate query parameter handling code
    let query_additions = if has_query_params || has_optional_params {
        let mut optional_checks = Vec::new();
        let mut required_additions = Vec::new();
        let mut has_required = false;

        for (name, ty) in field_names.iter().zip(field_types.iter()) {
            if !is_path_param(&name.to_string(), &url_pattern) {
                if is_option_type(ty) {
                    if let Some(inner_ty) = get_inner_type(ty) {
                        if is_vec_type(inner_ty) {
                            optional_checks.push(quote! {
                                if let Some(values) = &self.#name {
                                    if !values.is_empty() {
                                        for value in values {
                                            parsed_url.query_pairs_mut().append_pair(
                                                stringify!(#name),
                                                &value.to_string()
                                            );
                                        }
                                    }
                                }
                            });
                        } else {
                            optional_checks.push(quote! {
                                if let Some(value) = &self.#name {
                                    parsed_url.query_pairs_mut().append_pair(
                                        stringify!(#name),
                                        &value.to_string()
                                    );
                                }
                            });
                        }
                    }
                } else if is_vec_type(ty) {
                    optional_checks.push(quote! {
                        if !self.#name.is_empty() {
                            for value in &self.#name {
                                parsed_url.query_pairs_mut().append_pair(
                                    stringify!(#name),
                                    &value.to_string()
                                );
                            }
                        }
                    });
                } else {
                    has_required = true;
                    required_additions.push(quote! {
                        parsed_url.query_pairs_mut().append_pair(
                            stringify!(#name),
                            &self.#name.to_string()
                        );
                    });
                }
            }
        }

        // Combine required and optional parameters
        if has_required {
            quote! {
                #(#optional_checks)*
                #(#required_additions)*
            }
        } else {
            quote! {
                #(#optional_checks)*
                if parsed_url.query().map_or(true, str::is_empty) {
                    parsed_url.set_query(None);
                }
            }
        }
    } else {
        quote! {}
    };

    // Generate the final implementation
    let expanded = quote! {
        impl AsUrl for #name {
            fn as_url(&self, base_url: &str) -> Result<url::Url, url::ParseError> {
                let mut url = String::from(base_url);
                #(#segments)*

                let mut parsed_url = url::Url::parse(&url)?;
                #query_additions

                Ok(parsed_url)
            }
        }
    };

    TokenStream::from(expanded)
}
