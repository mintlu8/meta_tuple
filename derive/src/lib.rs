use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, Generics, Lifetime};

fn inject_static_bounds(input: &mut Generics) {
    for param in &mut input.params {
        if let GenericParam::Type(p) = param {
            p.bounds.push(syn::TypeParamBound::Lifetime(Lifetime::new("'static", Span::call_site())));
        }
    }
}

/// Make the type as a whole a `MetaTuple`, equivalent to wrapping it in `MetaItem`.
#[proc_macro_derive(MetaItem)]
pub fn derive_meta_item(tokens: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;
    inject_static_bounds(&mut input.generics);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        unsafe impl #impl_generics ::meta_tuple::MetaBox for #name #ty_generics #where_clause {
            fn as_erased(&self) -> ::meta_tuple::ErasedInner<'_> {
                ::meta_tuple::ErasedInner::Any(self)
            }
            fn as_erased_mut(&mut self) -> ::meta_tuple::ErasedInnerMut<'_> {
                ::meta_tuple::ErasedInnerMut::Any(self)
            }
            fn as_erased_ptr(&self) -> ::meta_tuple::ErasedInnerPtr<'_> {
                ::meta_tuple::ErasedInnerPtr::Any(self)
            }
        }
        
        unsafe impl #impl_generics ::meta_tuple::MetaTuple for #name #ty_generics #where_clause {
            fn get<__T: 'static>(&self) -> Option<&__T> {
                (self as &dyn ::core::any::Any).downcast_ref()
            }
            fn get_mut<__T: 'static>(&mut self) -> Option<&mut __T> {
                (self as &mut dyn ::core::any::Any).downcast_mut()
            }
            fn get_mut_ptr<__T: 'static>(&self) -> Option<*mut __T> {
                (self as &dyn ::core::any::Any).downcast_ref()
                    .map(|x| x as *const __T as *mut __T)
            }
        }
    }.into()
}

/// Make the type a `MetaTuple` of its fields.
#[proc_macro_derive(MetaTuple)]
pub fn derive_meta_tuple(tokens: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;
    inject_static_bounds(&mut input.generics);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let data_struct = match input.data {
        syn::Data::Struct(data_struct) => data_struct,
        _ => return quote! {compile_error!("Expected struct.")}.into()
    };
    let mut fields = Vec::new();
    match data_struct.fields {
        syn::Fields::Named(fields_named) => {
            for field in fields_named.named {
                fields.push(TokenTree::Ident(field.ident.unwrap()));
            }
        },
        syn::Fields::Unnamed(fields_unnamed) => {
            for (index, _) in fields_unnamed.unnamed.into_iter().enumerate() {
                fields.push(TokenTree::Literal(Literal::usize_unsuffixed(index)));
            }
        },
        syn::Fields::Unit => (),
    }

    let indices: Vec<_> = (0..fields.len()).collect();

    quote! {
        impl #impl_generics ::meta_tuple::TypeReflect for #name #ty_generics #where_clause {
            fn get_field(&self, index: usize) -> Option<&dyn ::core::any::Any> {
                match index {
                    #(#indices => Some(&self.#fields),)*
                    _ => None,
                }
            }

            fn get_field_mut(&mut self, index: usize) -> Option<&mut dyn ::core::any::Any> {
                match index {
                    #(#indices => Some(&mut self.#fields),)*
                    _ => None,
                }
            }
        }

        unsafe impl #impl_generics ::meta_tuple::MetaBox for #name #ty_generics #where_clause {
            fn as_erased(&self) -> ErasedInner<'_> {
                ::meta_tuple::ErasedInner::Struct(self)
            }

            fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
                ::meta_tuple::ErasedInnerMut::Struct(self)
            }

            fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
                ::meta_tuple::ErasedInnerPtr::Struct(self)
            }
        }
        
        unsafe impl #impl_generics ::meta_tuple::MetaTuple for #name #ty_generics #where_clause {
            fn get<__T: 'static>(&self) -> Option<&__T> {
                #(if let Some(result) = (self.#fields as &dyn ::core::any::Any).downcast_ref() {
                    return Some(result);
                })*
                None
            }
            fn get_mut<__T: 'static>(&mut self) -> Option<&mut __T> {
                #(if let Some(result) = (self.#fields as &mut dyn ::core::any::Any).downcast_mut() {
                    return Some(result);
                })*
                None
            }
            fn get_mut_ptr<__T: 'static>(&self) -> Option<*mut __T> {
                #(if let Some(result) = (self.#fields as &dyn ::core::any::Any).downcast_ref() {
                    return Some(result as *const __T as *mut __T);
                })*
                None
            }
        }
    }.into()
}