extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(MetaEnum)]
pub fn derive_meta_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let key_lit = name.to_string();

    let expanded = quote! {
        impl MetaEnumImpl for #name {
            const META_KEY: &'static str = #key_lit;
        }
    };
    TokenStream::from(expanded)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn igStruct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let meta_struct_name =
        syn::Ident::new(&format!("{}MetaField", struct_name), struct_name.span());

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named) => &named.named,
            _ => panic!("Expected named fields"),
        },
        _ => panic!("#[igStruct] only supports structs"),
    };

    // Generate reading code for each field (simplified)
    let read_fields = fields.iter().map(|field| {
        let name = field.ident.as_ref().expect("internal igStruct error #1");
        let ty = &field.ty;
        if quote!(#ty).to_string().contains("Option < String") {
            quote! {
                let string_meta_field = igStringMetaField;

                let #name = string_meta_field.value_from_igz(handle, endian, ctx, registry, metadata_manager)
                    .map(|s| Some(s.read().unwrap().downcast_ref::<Arc<str>>().expect("igStruct string downcast failed.").to_string()))
                    .unwrap_or(None);
            }
        } else if quote!(#ty).to_string() == "u32" {
            quote! {
                let #name = read_u32(handle, endian).expect("igStruct impl u32 decoding failed");
            }
        } else {
            quote! {
                let #name = todo!("Unsupported field type");
            }
        }
    });

    let init_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote!(#name,)
    });

    let expanded = quote! {
        #input

        pub struct #meta_struct_name;

        impl igMetaField for #meta_struct_name {
            fn type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<#struct_name>()
            }

            fn value_from_igz(
                &self,
                handle: &mut std::io::Cursor<Vec<u8>>,
                endian: &Endian,
                ctx: &IgzLoaderContext,
                registry: &igMetafieldRegistry,
                metadata_manager: &igMetadataManager
            ) -> Option<igAny> {
                use crate::util::byteorder_fixes::*;
                #(#read_fields)*
                Some(std::sync::Arc::new(std::sync::RwLock::new(#struct_name {
                    #(#init_fields)*
                })))
            }

            fn value_into_igz(
                &self,
                handle: &mut std::io::Cursor<Vec<u8>>,
                endian: &Endian,
                ctx: &mut IgzSaverContext,
            ) -> Result<(), IgzSaverError> {
                todo!()
            }

            fn value_from_igx(
                &self,
                handle: &mut std::io::Cursor<Vec<u8>>,
                endian: &Endian,
                ctx: &mut IgxLoaderContext,
            ) -> Option<igAny> {
                todo!()
            }

            fn value_into_igx(
                &self,
                handle: &mut std::io::Cursor<Vec<u8>>,
                endian: &Endian,
                ctx: &mut IgxSaverContext,
            ) -> Result<(), IgxSaverError> {
                todo!()
            }

            fn value_from_igb(
                &self,
                handle: &mut std::io::Cursor<Vec<u8>>,
                endian: &Endian,
                ctx: &mut IgbLoaderContext,
            ) -> Option<igAny> {
                todo!()
            }

            fn value_into_igb(
                &self,
                handle: &mut std::io::Cursor<Vec<u8>>,
                endian: &Endian,
                ctx: &mut IgbSaverContext,
            ) -> Result<(), IgbSaverError> {
                todo!()
            }

            fn platform_size(&self, ig_metadata_manager: &igMetadataManager, platform: IG_CORE_PLATFORM) -> u32 {
                todo!("platform_size")
            }

            fn platform_alignment(&self, ig_metadata_manager: &igMetadataManager, platform: IG_CORE_PLATFORM) -> u32 {
                todo!("platform_alignment")
            }
        }
    };

    TokenStream::from(expanded)
}
