use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Ident, ItemStruct};

struct SingletonArgs {
    use_arc: bool,
}

impl Parse for SingletonArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(SingletonArgs { use_arc: false });
        }

        let ident: Ident = input.parse()?;
        if ident == "arc" {
            Ok(SingletonArgs { use_arc: true })
        } else {
            Err(syn::Error::new(
                ident.span(),
                "Expected 'arc' or no arguments",
            ))
        }
    }
}

/// A macro to define a singleton struct.
///
/// There are two modes:
/// - Default(`#[singleton]`): Returns a static reference to `Self`.
/// - `#[singleton(arc)]`: Returns an `&'static Arc<Self>`, allowing for shared ownership across threads.
///
/// Usage:
/// ```
/// use qsingleton::singleton;
///
/// #[singleton]
/// #[derive(Debug)]
/// struct Config {
///    name: String,
///    version: String,
/// }  
///
/// #[singleton(arc)]
/// #[derive(Debug)]
/// struct Database {
///    connection_string: String,
///    pool_size: usize,
/// }
/// ```
#[proc_macro_attribute]
pub fn singleton(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as SingletonArgs);
    let input_struct = parse_macro_input!(input as ItemStruct);
    let name = &input_struct.ident;

    // Create unique global variable name to prevent conflicts
    let instance_name = syn::Ident::new(
        &format!("{}_INSTANCE", name.to_string().to_uppercase()),
        name.span(),
    );

    let impl_block = if args.use_arc {
        quote! {
            static #instance_name: ::std::sync::OnceLock<std::sync::Arc<#name>> = ::std::sync::OnceLock::new();
            impl #name {
                /// Get the global singleton instance as Arc
                pub fn global() -> &'static ::std::sync::Arc<Self> {
                    #instance_name.get().expect(&format!(
                        "Singleton '{}' not initialized. Call init() first.",
                        stringify!(#name)
                    ))
                }

                /// Initialize the singleton instance
                pub fn init(instance: Self) {
                    #instance_name.set(::std::sync::Arc::new(instance)).expect(&format!(
                        "Singleton '{}' already initialized",
                        stringify!(#name)
                    ))
                }
            }
        }
    } else {
        quote! {
            static #instance_name: ::std::sync::OnceLock<#name> = ::std::sync::OnceLock::new();
            impl #name {
                /// Get the global singleton instance
                pub fn global() -> &'static Self {
                    #instance_name.get().expect(&format!(
                        "Singleton '{}' not initialized. Call init() first.",
                        stringify!(#name)
                    ))
                }

                /// Initialize the singleton instance
                pub fn init(instance: Self) {
                    #instance_name.set(instance).expect(&format!(
                        "Singleton '{}' already initialized",
                        stringify!(#name)
                    ))
                }
            }
        }
    };

    let expanded = quote! {
        #input_struct
        #impl_block
    };

    TokenStream::from(expanded)
}
