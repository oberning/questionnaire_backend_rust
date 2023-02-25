use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ToJson)]
pub fn gimme_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_to_json(&ast)
}

fn impl_to_json(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {

        // pub trait HelloMacro {
        //     fn hello(&self) -> String;
        // }
        
        // impl HelloMacro for #name {
        //     fn hello(&self) -> String {
        //         format!( "Hello, Macro! My name is {}!", &self.text )
        //     }
        // }

        pub trait ToJson {
            fn to_json(&self) -> String where Self: Serialize;
        }

        impl<'a> ToJson for #name<'a> {
            fn to_json(&self) -> String where Self: Serialize {
                serde_json::to_string(&self).unwrap()
            }
        }


    };
    gen.into()
}
