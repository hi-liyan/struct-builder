use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as syn::DeriveInput);

    match do_expand(&st) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into()
    }
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;

/// 从 DeriveInput 解析结构体中的 fields
fn get_fields_from_derive_input(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named, .. }), .. }) = &st.data {
        Ok(named)
    } else {
        Err(syn::Error::new_spanned(st, "Only structs are supported"))
    }
}

/// 生成 Builder 结构体的 fields 代码
fn gen_builder_struct_fields_token(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let field_idents: Vec<_> = fields.iter().map(|it| { &it.ident }).collect();
    let field_types: Vec<_> = fields.iter().map(|it| { &it.ty }).collect();

    let ret = quote! {
      #(#field_idents: #field_types),*
    };

    Ok(ret)
}

/// 生成 Builder 实例中每一个 field 的初始化代码
fn gen_builder_struct_fields_init_token(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let field_idents: Vec<_> = fields.iter().map(|it| { &it.ident }).collect();

    let ret = quote! {
        #(#field_idents: std::option::Option::None),*
    };

    Ok(ret)
}

/// 生成 Builder impl 中每个字段的 set 函数
fn gen_builder_impl_field_fn_token(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let fn_tokens: Vec<_> = fields.iter().map(|it| {
        let ident = &it.ident;
        let ty = &it.ty;
        quote! {
            pub fn #ident(mut self, #ident: #ty) -> Self {
                self.#ident = #ident;
                self
            }
        }
    }).collect();

    let ret = quote! {
        #(#fn_tokens)*
    };

    Ok(ret)
}

/// 生成 Builder impl 中的 build 函数
fn gen_builder_impl_build_fn_token(fields: &StructFields, struct_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let field_idents: Vec<_> = fields.iter().map(|it| { &it.ident }).collect();

    let ret = quote! {
        pub fn build(self) -> #struct_ident {
            #struct_ident {
                #(#field_idents: self.#field_idents),*
            }
        }
    };

    Ok(ret)
}

fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let struct_name_ident = &st.ident;
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.ident.span());

    // 获取结构体的 Fields
    let struct_fields = get_fields_from_derive_input(st)?;

    // 生成代码
    let builder_struct_fields_token = gen_builder_struct_fields_token(struct_fields)?;
    let builder_struct_fields_init_token = gen_builder_struct_fields_init_token(struct_fields)?;
    let builder_impl_field_fn_token = gen_builder_impl_field_fn_token(struct_fields)?;
    let builder_impl_build_fn_token = gen_builder_impl_build_fn_token(struct_fields, &struct_name_ident)?;

    let ret = quote! {
        pub struct #builder_name_ident {
            #builder_struct_fields_token
        }
        impl #struct_name_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident {
                    #builder_struct_fields_init_token
                }
            }
        }

        impl #builder_name_ident {
            #builder_impl_field_fn_token
            #builder_impl_build_fn_token
        }
    };

    Ok(ret)
}
