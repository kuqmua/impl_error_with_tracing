#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

use proc_macro_helpers::global_variables::hardcode::ERROR_ENUM_NAME;
use proc_macro_helpers::global_variables::hardcode::ORIGIN_NAME;
use proc_macro_helpers::global_variables::hardcode::WRAPPER_NAME;

#[proc_macro_derive(ImplErrorWithTracingFromTufaCommon)]
pub fn derive_impl_error_with_tracing_from_tufa_common(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::TufaCommon)
}

#[proc_macro_derive(ImplErrorWithTracingFromCrate)]
pub fn derive_impl_error_with_tracing_from_crate(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::Crate)
}

fn generate(
    input: proc_macro::TokenStream,
    path: proc_macro_helpers::path::Path,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplErrorWithTracing syn::parse(input) failed");
    let fields = match ast.data {
        syn::Data::Struct(struct_item) => struct_item.fields,
        _ => panic!("ImplErrorWithTracing only works on structs"),
    };
    let ident = &ast.ident;
    let fields_named = match fields {
        syn::Fields::Named(fields_named) => fields_named,
        _ => panic!("ImplErrorWithTracing only works with named fields"),
    };
    match fields_named.named.len() {
        2 => (),
        _ => {
            panic!("ImplErrorWithTracing fields_named.len() != 2")
        }
    }
    let source_type_ident = match &fields_named.named[0].ty {
        syn::Type::Path(type_path) => type_path,
        _ => panic!("ImplErrorWithTracing only works on structs fields with  syn::Type::Path type"),
    };
    let first_source_type_ident = source_type_ident.path.segments[0].ident.clone();
    let first_source_type_ident_as_string = format!("{}", first_source_type_ident);
    let source_place_type_source_token_stream =
        format!("{path}::config_mods::source_place_type::SourcePlaceType::Source")
            .parse::<proc_macro2::TokenStream>()
            .expect("path parse failed");
    let source_place_type_github_token_stream =
        format!("{path}::config_mods::source_place_type::SourcePlaceType::Github")
            .parse::<proc_macro2::TokenStream>()
            .expect("path parse failed");
    let source_place_type_none_token_stream =
        format!("{path}::config_mods::source_place_type::SourcePlaceType::None")
            .parse::<proc_macro2::TokenStream>()
            .expect("path parse failed");
    let with_tracing_token_stream = format!("{path}::traits::with_tracing::WithTracing")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let where_was_token_stream = format!("{path}::common::where_was::WhereWas")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let source_place_type_token_stream =
        format!("{path}::config_mods::source_place_type::SourcePlaceType")
            .parse::<proc_macro2::TokenStream>()
            .expect("path parse failed");
    let tracing_token_stream = format!("{path}::config_mods::log_type::LogType::Tracing")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let stack_token_stream = format!("{path}::config_mods::log_type::LogType::Stack")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let none_token_stream = format!("{path}::config_mods::log_type::LogType::None")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let error_color_token_stream = format!("{path}::traits::get_color::ErrorColorBold")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let error_and_where_was_init = if first_source_type_ident_as_string == *"Vec" {
        let ident_as_string = match source_type_ident.path.segments[0].arguments.clone() {
            syn::PathArguments::None => {
                panic!("ImplErrorWithTracing does not work with syn::PathArguments::None")
            }
            syn::PathArguments::AngleBracketed(angle_bracketed) => {
                match angle_bracketed.args.len() {
                    1 => match angle_bracketed.args[0].clone() {
                        syn::GenericArgument::Type(type_handle) => match type_handle {
                            syn::Type::Path(type_path) => match type_path.path.segments.len() {
                                1 => type_path.path.segments[0].ident.to_string(),
                                _ => panic!(
                                    "ImplErrorWithTracing type_path.path.segments.len() != 1"
                                ),
                            },
                            _ => panic!("ImplErrorWithTracing works only with syn::Type::Path"),
                        },
                        _ => panic!(
                            "ImplErrorWithTracing works only with syn::GenericArgument::Type"
                        ),
                    },
                    _ => panic!("ImplErrorWithTracing 1 angle_bracketed.args.len() != 1"),
                }
            }
            syn::PathArguments::Parenthesized(_) => {
                panic!("ImplErrorWithTracing does not work with syn::PathArguments::Parenthesized")
            }
        };
        if ident_as_string.contains(WRAPPER_NAME) && ident_as_string.contains(ORIGIN_NAME) {
            panic!(
                "ImplErrorWithTracing - ident name {} contains {} and {}",
                ident_as_string, WRAPPER_NAME, ORIGIN_NAME
            );
        } else if ident_as_string.contains(WRAPPER_NAME) {
            quote::quote! {

                match source_place_type {
                    #source_place_type_source_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|e| e.get_log_where_was(source_place_type, CONFIG.log_type, e.get_source()))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_github_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|e| e.get_log_where_was(source_place_type, CONFIG.log_type, e.get_source()))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_none_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|e| format!("{}, ", e.get_source()))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                };
            }
        } else if ident_as_string.contains(ORIGIN_NAME) {
            quote::quote! {

              match source_place_type {
                  #source_place_type_source_token_stream => {
                      let mut error_handle = source
                      .iter()
                      .map(|e| format!("{}, ", e))
                      .fold(String::from(""), |mut acc, elem| {
                          acc.push_str(&elem);
                          acc
                      });
                      if !error_handle.is_empty() {
                          error_handle.pop();
                          error_handle.pop();
                      }
                      let where_was_handle = where_was.file_line_column();
                      match CONFIG.log_type {
                          #tracing_token_stream => {
                              tracing::error!(error = error_handle);
                          }
                          #stack_token_stream => {
                              println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                          }
                          #none_token_stream => (),
                      }
                  }
                  #source_place_type_github_token_stream => {
                      let mut error_handle = source
                      .iter()
                      .map(|e| format!("{}, ", e))
                      .fold(String::from(""), |mut acc, elem| {
                          acc.push_str(&elem);
                          acc
                      });
                      if !error_handle.is_empty() {
                          error_handle.pop();
                          error_handle.pop();
                      }
                      let where_was_handle = where_was.github_file_line_column(&where_was.git_info);
                      match CONFIG.log_type {
                          #tracing_token_stream => {
                              tracing::error!(error = error_handle);
                          }
                          #stack_token_stream => {
                              println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                          }
                          #none_token_stream => (),
                      }
                  }
                  #source_place_type_none_token_stream => {
                      let mut error_handle = source
                      .iter()
                      .map(|e| format!("{}, ", e))
                      .fold(String::from(""), |mut acc, elem| {
                          acc.push_str(&elem);
                          acc
                      });
                      if !error_handle.is_empty() {
                          error_handle.pop();
                          error_handle.pop();
                      }
                      match CONFIG.log_type {
                          #tracing_token_stream => {
                              tracing::error!(error = error_handle);
                          }
                          #stack_token_stream => {
                              println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                          }
                          #none_token_stream => (),
                      }
                  }
              };
            }
        } else {
            panic!(
                "ImplErrorWithTracing - ident name {} does not contain {} or {}",
                ident_as_string, WRAPPER_NAME, ORIGIN_NAME
            );
        }
    } else if first_source_type_ident_as_string == *"HashMap" {
        let ident_as_string = match source_type_ident.path.segments[0].arguments.clone() {
            syn::PathArguments::None => {
                panic!("ImplErrorWithTracing does not work with syn::PathArguments::None")
            }
            syn::PathArguments::AngleBracketed(angle_bracketed) => {
                match angle_bracketed.args.len() {
                    2 => match angle_bracketed.args[1].clone() {
                        syn::GenericArgument::Type(type_handle) => match type_handle {
                            syn::Type::Path(type_path) => match type_path.path.segments.len() {
                                1 => type_path.path.segments[0].ident.to_string(),
                                _ => panic!(
                                    "ImplErrorWithTracing type_path.path.segments.len() != 1"
                                ),
                            },
                            _ => panic!("ImplErrorWithTracing works only with syn::Type::Path"),
                        },
                        _ => panic!(
                            "ImplErrorWithTracing works only with syn::GenericArgument::Type"
                        ),
                    },
                    _ => panic!("ImplErrorWithTracing 2 angle_bracketed.args.len() != 1"),
                }
            }
            syn::PathArguments::Parenthesized(_) => {
                panic!("ImplErrorWithTracing does not work with syn::PathArguments::Parenthesized")
            }
        };
        if ident_as_string.contains(WRAPPER_NAME) && ident_as_string.contains(ORIGIN_NAME) {
            panic!(
                "ImplErrorWithTracing - ident name {} contains {} and {}",
                ident_as_string, WRAPPER_NAME, ORIGIN_NAME
            );
        } else if ident_as_string.contains(WRAPPER_NAME) || ident_as_string.contains(ORIGIN_NAME) {
            quote::quote! {

                match source_place_type {
                    #source_place_type_source_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|(key, e)| e.get_log_where_was(source_place_type, CONFIG.log_type, format!("{} {}", key, e.get_source())))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_github_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|(key, e)| e.get_log_where_was(source_place_type, CONFIG.log_type, format!("{} {}", key, e.get_source())))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_none_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|(key, e)| format!("{} {}, ", key, e.get_source()))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                };
            }
        } else {
            quote::quote! {

                match source_place_type {
                    #source_place_type_source_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|(key, e)| format!("{} {}, ", key, e))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        let where_was_handle = where_was.file_line_column();
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_github_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|(key, e)| format!("{} {}, ", key, e))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        let where_was_handle = where_was.github_file_line_column(&where_was.git_info);
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_none_token_stream => {
                        let mut error_handle = source
                        .iter()
                        .map(|(key, e)| format!("{} {}, ", key, e))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !error_handle.is_empty() {
                            error_handle.pop();
                            error_handle.pop();
                        }
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                };
            }
        }
    } else if first_source_type_ident_as_string.contains(ERROR_ENUM_NAME) {
        if first_source_type_ident_as_string.contains(WRAPPER_NAME)
            && first_source_type_ident_as_string.contains(ORIGIN_NAME)
        {
            panic!(
                "ImplErrorWithTracing - ident name {} contains {} and {}",
                first_source_type_ident_as_string, WRAPPER_NAME, ORIGIN_NAME
            );
        } else if first_source_type_ident_as_string.contains(WRAPPER_NAME) {
            quote::quote! {

                match source_place_type {
                    #source_place_type_source_token_stream => {
                        let error_handle = source.get_log_with_additional_where_was(
                            &where_was,
                            source_place_type,
                            source.get_source(),
                            CONFIG.log_type
                        );
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_github_token_stream => {
                        let error_handle = source.get_log_with_additional_where_was(
                            &where_was,
                            source_place_type,
                            source.get_source(),
                            CONFIG.log_type
                        );
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_none_token_stream => {
                        let error_handle = source.get_source();
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                };
            }
        } else if first_source_type_ident_as_string.contains(ORIGIN_NAME) {
            quote::quote! {

                match source_place_type {
                    #source_place_type_source_token_stream => {
                        let error_handle = source.get_source();
                        let where_was_handle = where_was.file_line_column();
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_github_token_stream => {
                        let error_handle = source.get_source();
                        let where_was_handle = where_was.github_file_line_column(&where_was.git_info);
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                    #source_place_type_none_token_stream => {
                        let error_handle = source.get_source();
                        match CONFIG.log_type {
                            #tracing_token_stream => {
                                tracing::error!(error = error_handle);
                            }
                            #stack_token_stream => {
                                println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                            }
                            #none_token_stream => (),
                        }
                    }
                }
            }
        } else {
            panic!(
                "ImplErrorWithTracing - ident name {} does not contain {} or {}",
                first_source_type_ident_as_string, WRAPPER_NAME, ORIGIN_NAME
            );
        }
    } else {
        quote::quote! {
            match source_place_type {
                #source_place_type_source_token_stream => {
                    let error_handle = format!("{} {}", where_was.file_line_column(), source);
                    match CONFIG.log_type {
                        #tracing_token_stream => {
                            tracing::error!(error = error_handle);
                        }
                        #stack_token_stream => {
                            println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                        }
                        #none_token_stream => (),
                    }
                }
                #source_place_type_github_token_stream => {
                    let error_handle = format!("{} {}", where_was.github_file_line_column(&where_was.git_info), source);
                    match CONFIG.log_type {
                        #tracing_token_stream => {
                            tracing::error!(error = error_handle);
                        }
                        #stack_token_stream => {
                            println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                        }
                        #none_token_stream => (),
                    }
                }
                #source_place_type_none_token_stream => {
                    let error_handle = format!("{}", source);
                    match CONFIG.log_type {
                        #tracing_token_stream => {
                            tracing::error!(error = error_handle);
                        }
                        #stack_token_stream => {
                            println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
                        }
                        #none_token_stream => (),
                    }
                }
            }
        }
    };
    let gen = quote::quote! {
        use ansi_term::Colour::RGB;
        impl #with_tracing_token_stream<#source_type_ident> for #ident {
            fn with_tracing(
                source: #source_type_ident,
                where_was: #where_was_token_stream,
                source_place_type: &#source_place_type_token_stream,
            ) -> Self {
                use #error_color_token_stream;
                #error_and_where_was_init
                Self { source, where_was }
            }
        }
    };
    gen.into()
}
