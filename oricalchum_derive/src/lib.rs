use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(TrackActor)]
pub fn track_actor_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_track_actor(&ast)
}

fn impl_track_actor(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("Should be derived for structs"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let gen = quote! {
        impl TrackActor for #name {
            fn log(&self) {
                 #(
                    println!("{}: {:?}", stringify!(#field_names), &self.#field_names);
                )*
            }
        }
    };

    gen.into()
}