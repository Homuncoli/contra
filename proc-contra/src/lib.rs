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
    let closing_field = ser_fields.next_back()
        .map(|f| Some(quote!(ser.serialize_field(stringify!(#f), &self.#f, &lib_contra::position::Position::Closing )?; ))).into_iter();
    let trailing_fields = ser_fields
        .map(|f| Some(quote!(ser.serialize_field(stringify!(#f), &self.#f, &lib_contra::position::Position::Trailing)?; ))).into_iter();
    let ser_fields = trailing_fields.chain(closing_field.into_iter()).filter(|f| f.is_some());

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

#[proc_macro_derive(Deserialize)]
pub fn impl_deserialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let c_ident = ast.ident;
    let n_fields = ast.attrs.len();
    let field_idents = match ast.data {
        syn::Data::Struct(str) => str.fields.into_iter()
                                                        .map(|f| f.ident)
                                                        .filter(|f| f.is_some())
                                                        .map(|f| f.unwrap()),
        syn::Data::Enum(_) => panic!("cannot deserialize enums as of yet"),
        syn::Data::Union(_) => panic!("cannot deserialize unions as of yet"),
    };

    let mut des_fields = field_idents.clone();
    let closing_field = des_fields.next_back()
        .map(|f| Some(quote!(let #f = des.deserialize_field(stringify!(#f))?;))).into_iter();
    let trailing_fields = des_fields
        .map(|f| Some(quote!(let #f = des.deserialize_field(stringify!(#f))?;))).into_iter();
    let des_fields = trailing_fields.chain(closing_field).filter(|f| f.is_some());

    quote!(
        impl lib_contra::deserialize::Deserialize for #c_ident {
            fn deserialize<D: lib_contra::deserializer::Deserializer>(des: &mut D) -> Result<Self, lib_contra::error::AnyError> {
                des.deserialize_struct_begin(stringify!(#c_ident), #n_fields)?;
                
                #(#des_fields)*
    
                des.deserialize_struct_end(stringify!(#c_ident))?;

                Ok(#c_ident {
                    #(#field_idents),*
                })
            }
        }
    ).into()
}
