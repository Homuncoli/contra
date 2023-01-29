use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

#[proc_macro_derive(Serialize)]
pub fn impl_serialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let c_ident = ast.ident;
    let n_fields = ast.attrs.len();
    let mut ser_fields = match ast.data {
        syn::Data::Struct(str) => str.fields.into_iter()
                                                        .map(|f| f.ident)
                                                        .filter(|f| f.is_some())
                                                        .map(|f| f.unwrap()),
        syn::Data::Enum(_) => panic!("cannot serialize enums as of yet"),
        syn::Data::Union(_) => panic!("cannot serialize unions as of yet"),
    };
    let closing_field = ser_fields.next_back().map(|f| Some(quote!(ser.serialize_field(stringify!(#f), &self.#f, &lib_contra::position::Position::Closing)?; ))).into_iter();
    let trailing_fields = ser_fields
        .map(|f| Some(quote!(ser.serialize_field(stringify!(#f), &self.#f, &lib_contra::position::Position::Trailing)?; )));
    let ser_fields = trailing_fields.chain(closing_field.into_iter()).filter(|f| f.is_some());

    println!("n_fields: {}", n_fields);

    quote!(
        impl lib_contra::serialize::Serialize for #c_ident {
            fn serialize<S: lib_contra::serializer::Serializer>(&self, ser: &mut S, pos: &lib_contra::position::Position) -> lib_contra::error::SuccessResult {
                ser.begin_struct(stringify!(#c_ident), #n_fields)?;

                #(#ser_fields)*
                
                ser.end_struct(stringify!(#c_ident), pos)?;
                
                Ok(())
            }
        }
    ).into()
}
