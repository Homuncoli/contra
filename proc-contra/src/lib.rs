//! Macro implementations for [contra](https://docs.rs/contra)
//!
//! Provides the derive macros for the serialization and deserialization of any arbitrary object.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, DeriveInput};

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

    match ast.data {
        syn::Data::Struct(decl) => gen_struct_serialize(ast.ident, decl),
        syn::Data::Enum(decl) => gen_enum_serialize(ast.ident, decl),
        syn::Data::Union(_) => todo!(),
    }
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

    match ast.data {
        syn::Data::Struct(decl) => gen_struct_deserialize(ast.ident, decl),
        syn::Data::Enum(decl) => gen_enum_deserialize(ast.ident, decl),
        syn::Data::Union(_) => todo!(),
    }
}

fn gen_struct_serialize(ident: syn::Ident, decl: DataStruct) -> TokenStream {
    let c_ident = ident;
    let n_fields = decl.fields.len();
    let mut ser_fields = decl
        .fields
        .into_iter()
        .map(|f| f.ident)
        .filter(|f| f.is_some())
        .map(|f| f.unwrap());
    let closing_field = ser_fields.next_back()
        .map(|f| Some(quote!(ser.serialize_field(stringify!(#f), &self.#f, &contra::lib_contra::position::Position::Closing )?; ))).into_iter();
    let trailing_fields = ser_fields
        .map(|f| Some(quote!(ser.serialize_field(stringify!(#f), &self.#f, &contra::lib_contra::position::Position::Trailing)?; ))).into_iter();
    let ser_fields = trailing_fields
        .chain(closing_field.into_iter())
        .filter(|f| f.is_some());

    quote!(
        impl contra::lib_contra::serialize::Serialize for #c_ident {
            fn serialize<S: contra::lib_contra::serialize::Serializer>(&self, ser: &mut S, _pos: &contra::lib_contra::position::Position) -> contra::lib_contra::error::SuccessResult {
                ser.begin_struct(stringify!(#c_ident), #n_fields)?;

                #(#ser_fields)*

                ser.end_struct(stringify!(#c_ident))?;

                Ok(())
            }
        }
    ).into()
}

fn gen_enum_serialize(ident: syn::Ident, decl: DataEnum) -> TokenStream {
    let e_ident = ident;
    let variants = decl.variants.into_iter().map(|v| v.ident);

    let ser_variants = variants
        .clone()
        .map(|v| quote! { #e_ident::#v => ser.serialize_str(stringify!(#v)) });

    quote!(
        impl contra::lib_contra::serialize::Serialize for #e_ident {
            fn serialize<S: contra::lib_contra::serialize::Serializer>(&self, ser: &mut S, _pos: &contra::lib_contra::position::Position) -> contra::lib_contra::error::SuccessResult {
                match self {
                    #(#ser_variants,)*
                }
            }
        }
    ).into()
}

fn gen_enum_deserialize(ident: syn::Ident, decl: DataEnum) -> TokenStream {
    let e_ident = ident;
    let variants = decl.variants.into_iter().map(|v| v.ident);

    let parse_variants = variants
        .clone()
        .map(|v| quote! { stringify!(#v) => Ok(#e_ident::#v) });

    quote! {
        impl contra::lib_contra::deserialize::Deserialize for #e_ident {
            fn deserialize<D: contra::lib_contra::deserialize::Deserializer>(des: D) -> Result<Self, contra::lib_contra::error::AnyError> {
                struct EnumVisitor {}
                impl contra::lib_contra::deserialize::Visitor for EnumVisitor {
                    type Value = #e_ident;

                    fn expected_a(self) -> String {
                        concat!(stringify!(#e_ident), " variant").to_string()
                    }

                    fn visit_str(self, v: &str) -> Result<Self::Value, contra::lib_contra::error::AnyError> {
                        match v {
                            #(#parse_variants,)*
                            err => Err(format!("invalid {} variant \"{}\"", stringify!(#e_ident), err).into())
                        }
                    }
                }

                des.deserialize_str(EnumVisitor {})
            }
        }
    }.into()
}

fn gen_struct_deserialize(ident: syn::Ident, decl: DataStruct) -> TokenStream {
    let c_ident = ident;
    let f_idents = decl
        .fields
        .into_iter()
        .map(|f| f.ident)
        .filter(|f| f.is_some())
        .map(|f| f.unwrap());

    let field_enum_decl = f_idents.clone().map(|i| quote! { #i });
    let field_enum_parse = f_idents
        .clone()
        .map(|i| quote! { stringify!(#i) => Ok(Field::#i) });
    let tmp_field_decl = f_idents.clone().map(|i| quote! { let mut #i = None });
    let tmp_field_parse = f_idents.clone().map(|i| {
        quote! {
            Field::#i => {
                if #i.is_some() {
                    return Err(concat!("duplicate field ", stringify!(#i)).into());
                }
                #i = Some(map.next_value()?)
            }
        }
    });
    let tmp_field_result = f_idents
        .clone()
        .map(|i| quote! { let #i = #i.ok_or_else(|| concat!("missing field ", stringify!(#i)))? });
    let tmp_field_initializer_list = f_idents.clone().map(|i| quote! { #i });

    quote!(
        impl contra::lib_contra::deserialize::Deserialize for #c_ident {
            fn deserialize<D: contra::lib_contra::deserialize::Deserializer>(de: D) -> Result<Self, contra::lib_contra::error::AnyError> {
                enum Field {
                    #(#field_enum_decl,)*
                }
                impl contra::lib_contra::deserialize::Deserialize for Field {
                    fn deserialize<D: contra::lib_contra::deserialize::Deserializer>(de: D) -> Result<Self, contra::lib_contra::error::AnyError> {
                        struct FieldVisitor {}
                        impl contra::lib_contra::deserialize::Visitor for FieldVisitor {
                            type Value = Field;
                            fn expected_a(self) -> String {
                                concat!(stringify!(#c_ident), " field").into()
                            }
                            fn visit_str(self, v: &str) -> Result<Self::Value, contra::lib_contra::error::AnyError> {
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
                impl contra::lib_contra::deserialize::Visitor for StructVisitor {
                    type Value = #c_ident;
                    fn expected_a(self) -> String {
                        concat!(stringify!(#c_ident), " object").into()
                    }
                    fn visit_map<M: contra::lib_contra::deserialize::MapAccess>(self, mut map: M) -> Result<Self::Value, contra::lib_contra::error::AnyError> {
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
