use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Ident, ItemStruct, LitInt, Token};

struct SingletonArgs {
    use_arc: bool,
    sleep_ms: Option<u64>,
}

impl Parse for SingletonArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(SingletonArgs { 
                use_arc: false,
                sleep_ms: Some(500), // Default 500ms sleep
            });
        }

        let mut use_arc = false;
        let mut sleep_ms = Some(500u64); // Default value

        // Parse first argument
        let first_ident: Ident = input.parse()?;
        
        if first_ident == "arc" {
            use_arc = true;
            
            // Check if there's a comma for additional arguments
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                
                // Parse sleep_ms parameter
                if !input.is_empty() {
                    let param_name: Ident = input.parse()?;
                    if param_name == "sleep_ms" {
                        input.parse::<Token![=]>()?;
                        let sleep_value: LitInt = input.parse()?;
                        sleep_ms = Some(sleep_value.base10_parse()?);
                    } else {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Expected 'sleep_ms' parameter",
                        ));
                    }
                }
            }
        } else if first_ident == "sleep_ms" {
            // Parse sleep_ms parameter
            input.parse::<Token![=]>()?;
            let sleep_value: LitInt = input.parse()?;
            sleep_ms = Some(sleep_value.base10_parse()?);
        } else {
            return Err(syn::Error::new(
                first_ident.span(),
                "Expected 'arc' or 'sleep_ms' parameter",
            ));
        }

        Ok(SingletonArgs { use_arc, sleep_ms })
    }
}

/// A macro to define a singleton struct.
///
/// There are three modes:
/// - Default(`#[singleton]`): Returns a static reference to `Self` with 500ms default sleep.
/// - `#[singleton(arc)]`: Returns an `&'static Arc<Self>` with 500ms default sleep.
/// - `#[singleton(sleep_ms = 1000)]`: Custom sleep duration in milliseconds.
/// - `#[singleton(arc, sleep_ms = 1000)]`: Arc mode with custom sleep duration.
///
/// The sleep mechanism waits for the specified duration when the singleton is not yet initialized,
/// allowing time for initialization to complete in multi-threaded scenarios.
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
///
/// #[singleton(sleep_ms = 1000)]
/// #[derive(Debug)]
/// struct CustomSleep {
///    data: String,
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

    let sleep_duration_ms = args.sleep_ms.unwrap_or(500);

    let impl_block = if args.use_arc {
        quote! {
            static #instance_name: ::std::sync::OnceLock<::std::sync::Arc<#name>> = ::std::sync::OnceLock::new();
            impl #name {
                /// Get the global singleton instance as Arc
                /// 
                /// This method will sleep for the configured duration if the singleton is not yet initialized,
                /// allowing time for initialization to complete in concurrent scenarios.
                pub fn global() -> &'static ::std::sync::Arc<Self> {
                    const MAX_RETRIES: usize = 20; // Maximum 10 seconds with 500ms default sleep
                    let sleep_duration = ::std::time::Duration::from_millis(#sleep_duration_ms);
                    
                    for retry in 0..MAX_RETRIES {
                        if let Some(instance) = #instance_name.get() {
                            return instance;
                        }
                        
                        if retry < MAX_RETRIES - 1 {
                            ::std::thread::sleep(sleep_duration);
                        }
                    }
                    
                    panic!(
                        "Singleton '{}' not initialized after {} retries ({}ms each). Call init() first.",
                        stringify!(#name),
                        MAX_RETRIES,
                        #sleep_duration_ms
                    )
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
                /// 
                /// This method will sleep for the configured duration if the singleton is not yet initialized,
                /// allowing time for initialization to complete in concurrent scenarios.
                pub fn global() -> &'static Self {
                    const MAX_RETRIES: usize = 20; // Maximum 10 seconds with 500ms default sleep
                    let sleep_duration = ::std::time::Duration::from_millis(#sleep_duration_ms);
                    
                    for retry in 0..MAX_RETRIES {
                        if let Some(instance) = #instance_name.get() {
                            return instance;
                        }
                        
                        if retry < MAX_RETRIES - 1 {
                            ::std::thread::sleep(sleep_duration);
                        }
                    }
                    
                    panic!(
                        "Singleton '{}' not initialized after {} retries ({}ms each). Call init() first.",
                        stringify!(#name),
                        MAX_RETRIES,
                        #sleep_duration_ms
                    )
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
