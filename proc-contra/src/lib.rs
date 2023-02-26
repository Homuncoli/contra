//! Macro implementations for [contra](https://docs.rs/contra)
//!
//! Provides the derive macros for the serialization and deserialization of any arbitrary object.

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Derives the *Serialize* trait implementation
///
/// # Example
/// ```
/// use proc_contra::Serialize;
/// 
/// #[derive(Serialize)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32
/// }
/// ```
///
/// Expands into:
/// ```
/// use lib_contra::{serialize::Serialize, serialize::Serializer, position::Position, error::SuccessResult};
/// 
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32
/// }
/// 
/// impl Serialize for Point {
///     fn serialize<S: Serializer>(&self, ser: &mut S, _pos: &Position) -> SuccessResult {
///         ser.begin_struct("Point", 3)?;
///     
///         ser.serialize_field("x", &self.x, &Position::Trailing)?;
///         ser.serialize_field("y", &self.y, &Position::Trailing)?;
///         ser.serialize_field("z", &self.z, &Position::Closing)?;
///     
///         ser.end_struct("Point")?;
///     
///         Ok(())
///     }
/// }
/// ```
///
#[proc_macro_derive(Serialize)]
pub fn impl_serialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let c_ident = ast.ident;
    let n_fields = ast.attrs.len();
    let mut ser_fields = match ast.data {
        syn::Data::Struct(str) => str
            .fields
            .into_iter()
            .map(|f| f.ident)
            .filter(|f| f.is_some())
            .map(|f| f.unwrap()),
        syn::Data::Enum(_) => panic!("cannot serialize enums as of yet"),
        syn::Data::Union(_) => panic!("cannot serialize unions as of yet"),
    };
    let closing_field = ser_fields.next_back()
        .map(|f| Some(quote!(ser.serialize_field(stringify!(#f), &self.#f, &::lib_contra::position::Position::Closing )?; ))).into_iter();
    let trailing_fields = ser_fields
        .map(|f| Some(quote!(ser.serialize_field(stringify!(#f), &self.#f, &::lib_contra::position::Position::Trailing)?; ))).into_iter();
    let ser_fields = trailing_fields
        .chain(closing_field.into_iter())
        .filter(|f| f.is_some());

    quote!(
        impl ::lib_contra::serialize::Serialize for #c_ident {
            fn serialize<S: ::lib_contra::serialize::Serializer>(&self, ser: &mut S, _pos: &::lib_contra::position::Position) -> ::lib_contra::error::SuccessResult {
                ser.begin_struct(stringify!(#c_ident), #n_fields)?;

                #(#ser_fields)*

                ser.end_struct(stringify!(#c_ident))?;

                Ok(())
            }
        }
    ).into()
}

/// Derives the *Deserialize* trait implementation
///
/// # Example
/// ```
/// use proc_contra::Deserialize;
/// use lib_contra::{deserialize::Deserialize, position::Position, deserialize::Deserializer, error::AnyError};
/// #[derive(Deserialize)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32
/// }
/// ```
///
/// Expands into:
/// ```
/// use lib_contra::{deserialize::{MapAccess, Visitor, Deserialize}, position::Position, deserialize::Deserializer, error::AnyError};
/// 
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32
/// }
/// 
/// impl Deserialize for Point {
///     fn deserialize<D: Deserializer>(de: D) -> Result<Self, AnyError> {
///         enum Field {
///             x, y, z
///         }
///         impl Deserialize for Field {
///             fn deserialize<D: Deserializer>(de: D) -> Result<Self, AnyError> {
///                 struct FieldVisitor {}
///                 impl Visitor for FieldVisitor {
///                     type Value = Field;
///                     fn expected_a(self) -> String { "Point field".to_string() }
///                     fn visit_str(self, v: &str) -> Result<Self::Value, AnyError> { 
///                         match v {
///                             "x" => Ok(Field::x),
///                             "y" => Ok(Field::y),
///                             "z" => Ok(Field::z),
///                             val => Err(format!("unexpected Point field {}", val).into())
///                         }
///                     }
///                 }
///                 de.deserialize_str(FieldVisitor {})
///             }
///         }
/// 
///         struct PointVisitor {}
///         impl Visitor for PointVisitor {
///             type Value = Point;
///             fn expected_a(self) -> String { "Point object".to_string() }
///             fn visit_map<M: MapAccess>(self, mut map: M) -> Result<Self::Value, AnyError> {
///                 let mut x = None;
///                 let mut y = None;
///                 let mut z = None;
///                 
///                 while let Some(key) = map.next_key()? {
///                     match key {
///                         Field::x => { if x.is_some() { return Err("duplicate field x".into()); } x = Some(map.next_value()?) },
///                         Field::y => { if y.is_some() { return Err("duplicate field y".into()); } y = Some(map.next_value()?) },
///                         Field::z => { if z.is_some() { return Err("duplicate field z".into()); } z = Some(map.next_value()?) },
///                     }
///                 }
/// 
///                 let x = x.ok_or_else(|| "missing field x")?;
///                 let y = y.ok_or_else(|| "missing field y")?;
///                 let z = z.ok_or_else(|| "missing field z")?;
/// 
///                 Ok(Point {
///                     x, y, z
///                 })
///             }
///         }
/// 
///         de.deserialize_struct(PointVisitor {})
///     }
/// }
/// ```
#[proc_macro_derive(Deserialize)]
pub fn impl_deserialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let c_ident = ast.ident;
    let f_idents = match ast.data {
        syn::Data::Struct(str) => str
            .fields
            .into_iter()
            .map(|f| f.ident)
            .filter(|f| f.is_some())
            .map(|f| f.unwrap()),
        syn::Data::Enum(_) => panic!("cannot deserialize enums as of yet"),
        syn::Data::Union(_) => panic!("cannot deserialize unions as of yet"),
    };

    let field_enum_decl = f_idents.clone().map(|i| quote! { #i });
    let field_enum_parse = f_idents.clone().map(|i| quote! { stringify!(#i) => Ok(Field::#i) });
    let tmp_field_decl = f_idents.clone().map(|i| quote! { let mut #i = None });
    let tmp_field_parse = f_idents.clone().map(|i| quote! { 
        Field::#i => { 
            if #i.is_some() { 
                return Err(concat!("duplicate field ", stringify!(#i)).into()); 
            }
            #i = Some(map.next_value()?) 
        } 
    });
    let tmp_field_result = f_idents.clone().map(|i| quote! { let #i = #i.ok_or_else(|| concat!("missing field ", stringify!(#i)))? });
    let tmp_field_initializer_list = f_idents.clone().map(|i| quote! { #i });

    quote!(
        impl ::lib_contra::deserialize::Deserialize for #c_ident {
            fn deserialize<D: ::lib_contra::deserialize::Deserializer>(de: D) -> Result<Self, ::lib_contra::error::AnyError> {
                enum Field {
                    #(#field_enum_decl,)*
                }
                impl ::lib_contra::deserialize::Deserialize for Field {
                    fn deserialize<D: ::lib_contra::deserialize::Deserializer>(de: D) -> Result<Self, ::lib_contra::error::AnyError> {
                        struct FieldVisitor {}
                        impl ::lib_contra::deserialize::Visitor for FieldVisitor {
                            type Value = Field;
                            fn expected_a(self) -> String {
                                concat!(stringify!(#c_ident), " field").into()
                            }
                            fn visit_str(self, v: &str) -> Result<Self::Value, ::lib_contra::error::AnyError> {
                                match v {
                                    #(#field_enum_parse,)*
                                    val => Err(format!("unknown \"{}\" field for {}", val, stringify!(#c_ident)).into())
                                }
                            }
                        }
                        de.deserialize_str(FieldVisitor {})
                    }
                }
                
                struct StructVisitor {}
                impl ::lib_contra::deserialize::Visitor for StructVisitor {
                    type Value = #c_ident;
                    fn expected_a(self) -> String {
                        concat!(stringify!(#c_ident), " object").into()
                    }
                    fn visit_map<M: ::lib_contra::deserialize::MapAccess>(self, mut map: M) -> Result<Self::Value, ::lib_contra::error::AnyError> {
                        #(#tmp_field_decl;)*

                        while let Some(key) = map.next_key::<Field>()? {
                            match key {
                                #(#tmp_field_parse,)*
                            }
                        }

                        #(#tmp_field_result;)*

                        Ok(#c_ident {
                            #(#tmp_field_initializer_list,)*
                        })
                    }
                }

                de.deserialize_struct(StructVisitor {})
            }
        }
    ).into()
}
