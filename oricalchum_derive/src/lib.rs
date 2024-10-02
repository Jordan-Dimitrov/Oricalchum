use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(TrackActor)]
pub fn track_actor_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_track_actor(&ast)
}

fn impl_track_actor(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl TrackActor for #name {

        }
    };

    gen.into()
}