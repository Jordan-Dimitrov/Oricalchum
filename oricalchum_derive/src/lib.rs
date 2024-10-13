use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

/// Derive macro to implement the `TrackActor` trait for a struct.
///
/// This macro generates an implementation of the `TrackActor` trait for the
/// struct on which it is derived. The implementation includes a `log` method
/// that prints the names and values of all fields in the struct.
///
/// # Usage
///
/// To use this derive macro, simply annotate a struct with `#[derive(TrackActor)]`
/// and ensure that the struct contains fields. The macro will automatically
/// implement the `TrackActor` trait for that struct, allowing for easy logging
/// of its state.
///
/// # Example
///
/// ```rust
/// use oricalchum_derive::TrackActor;
/// #[derive(Debug, TrackActor)]
/// struct MyActor {
///     name: String,
///     value: i32,
/// }
///
/// let actor = MyActor { name: String::from("actor1"), value: 42 };
/// actor.log(); // This will print the names and values of `name` and `value`.
/// ```
///
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