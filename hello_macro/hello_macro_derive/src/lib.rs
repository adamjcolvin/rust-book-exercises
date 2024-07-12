use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the Rust code we want to generate
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
      impl HelloMacro for #name {
        fn hello_macro() {
          println!("Hello, Macro! My name is {}", stringify!(#name));
        }
      }
    };
    gen.into()
}
