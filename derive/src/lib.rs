use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenTree};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, GenericParam, Generics, Lifetime};

fn inject_static_bounds(input: &mut Generics) {
    for param in &mut input.params {
        if let GenericParam::Type(p) = param {
            p.bounds.push(syn::TypeParamBound::Lifetime(Lifetime::new(
                "'static",
                Span::call_site(),
            )));
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
        unsafe impl #impl_generics ::meta_tuple::MetaAny for #name #ty_generics #where_clause {
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
    }
    .into()
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
        _ => return quote! {compile_error!("Expected struct.")}.into(),
    };
    let mut fields = Vec::new();
    match data_struct.fields {
        syn::Fields::Named(fields_named) => {
            for field in fields_named.named {
                fields.push(TokenTree::Ident(field.ident.unwrap()));
            }
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            for (index, _) in fields_unnamed.unnamed.into_iter().enumerate() {
                fields.push(TokenTree::Literal(Literal::usize_unsuffixed(index)));
            }
        }
        syn::Fields::Unit => (),
    }

    let indices: Vec<_> = (0..fields.len()).collect();

    quote! {
        impl #impl_generics ::meta_tuple::MetaBundle for #name #ty_generics #where_clause {
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

        unsafe impl #impl_generics ::meta_tuple::MetaAny for #name #ty_generics #where_clause {
            fn as_erased(&self) -> ::meta_tuple::ErasedInner<'_> {
                ::meta_tuple::ErasedInner::Struct(self)
            }

            fn as_erased_mut(&mut self) -> ::meta_tuple::ErasedInnerMut<'_> {
                ::meta_tuple::ErasedInnerMut::Struct(self)
            }

            fn as_erased_ptr(&self) -> ::meta_tuple::ErasedInnerPtr<'_> {
                ::meta_tuple::ErasedInnerPtr::Struct(self)
            }
        }
        
        unsafe impl #impl_generics ::meta_tuple::MetaTuple for #name #ty_generics #where_clause {
            fn get<__T: 'static>(&self) -> Option<&__T> {
                #(if let Some(result) = (&self.#fields as &dyn ::core::any::Any).downcast_ref() {
                    return Some(result);
                })*
                None
            }
            fn get_mut<__T: 'static>(&mut self) -> Option<&mut __T> {
                #(if let Some(result) = (&mut self.#fields as &mut dyn ::core::any::Any).downcast_mut() {
                    return Some(result);
                })*
                None
            }
            fn get_mut_ptr<__T: 'static>(&self) -> Option<*mut __T> {
                #(if let Some(result) = (&self.#fields as &dyn ::core::any::Any).downcast_ref() {
                    return Some(result as *const __T as *mut __T);
                })*
                None
            }
        }
    }.into()
}

/// Fetch individual items from a `MetaTuple`.
/// 
/// # Syntax
/// 
/// ```
/// #[derive(MetaQuery)]
/// pub struct MyQuery<'t> {
///     int: &'t i32,
///     string: &'t String,
/// }
/// ```
/// 
/// # Semantics
/// 
/// Requires a generic lifetime, all generic types will be added `+ 'static` bound.
#[proc_macro_derive(MetaQuery)]
pub fn derive_meta_query(tokens: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;
    inject_static_bounds(&mut input.generics);
    let mut g2 = input.generics.clone();
    match g2.params.first_mut() {
        Some(GenericParam::Lifetime(lt)) => lt.lifetime = Lifetime::new("'__t", Span::call_site()),
        _ => return quote! {compile_error!("Expected at least one lifetime parameters.")}.into()
    }
    let gat_param = g2.split_for_impl().1;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let data_struct = match input.data {
        syn::Data::Struct(data_struct) => data_struct,
        _ => return quote! {compile_error!("Expected struct.")}.into(),
    };
    let mut fields = Vec::new();
    let mut types = Vec::new();
    let mut is_tuple = false;
    match data_struct.fields {
        syn::Fields::Named(fields_named) => {
            for field in fields_named.named {
                fields.push(field.ident.unwrap());
                types.push(field.ty);
            }
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            for (index, field) in fields_unnamed.unnamed.into_iter().enumerate() {
                fields.push(format_ident!("__v{}", index));
                types.push(field.ty);
                is_tuple = true;
            }
        }
        syn::Fields::Unit => (),
    }

    let mut type_comparisons = Vec::new();

    for x in 0..types.len() {
        for y in x+1..types.len() {
            let a = &types[x];
            let b = &types[y];
            type_comparisons.push(quote! {
                <#a as ::meta_tuple::MetaQuerySingle>::compatible::<#b>() 
            });
        }
    }

    if type_comparisons.is_empty() {
        type_comparisons.push(quote! {true});
    }

    let ref_init = if is_tuple {
        quote! {(
            #(<#types as ::meta_tuple::MetaQuery>::query_ref(input)?,)*
        )}
    } else {
        quote! {{
            #(#fields: <#types as ::meta_tuple::MetaQuery>::query_ref(input)?,)*
        }}
    };

    let dyn_init = if is_tuple {
        quote! {(
            #(<#types as ::meta_tuple::MetaQuery>::query_dyn_ref(input)?,)*
        )}
    } else {
        quote! {{
            #(#fields: <#types as ::meta_tuple::MetaQuery>::query_dyn_ref(input)?,)*
        }}
    };

    let ptr_init = if is_tuple {
        quote! {(
            #(unsafe {<#types as ::meta_tuple::MetaQuery>::from_ptr(#fields)},)*
        )}
    } else {
        quote! {{
            #(#fields: unsafe {<#types as ::meta_tuple::MetaQuery>::from_ptr(#fields)},)*
        }}
    };

    quote! {
        unsafe impl #impl_generics ::meta_tuple::MetaQuery for #name #ty_generics #where_clause {
            type Output<'__t> = #name #gat_param;

            fn query_ref<'__t, T: MetaTuple + ?Sized + '__t>(input: &'__t T) -> Option<Self::Output<'__t>> {
                Some(#name #ref_init)
            }

            fn query_dyn_ref<'__t>(input: &'__t dyn ::meta_tuple::MetaAny) -> Option<Self::Output<'__t>> {
                Some(#name #dyn_init)
            }

            type OutputPtr<'__t> = (#(<#types as ::meta_tuple::MetaQuery>::OutputPtr<'__t>,)*);

            unsafe fn from_ptr<'__t>(ptr: Self::OutputPtr<'__t>) -> Self::Output<'__t> {
                let (#(#fields,)*) = ptr;
                #name #ptr_init
            }

            fn query_mut_ptr<'__t, T: MetaTuple + ?Sized + '__t>(input: &'__t T) -> Option<Self::OutputPtr<'__t>> {
                Some((#(<#types as ::meta_tuple::MetaQuery>::query_mut_ptr(input)?,)*))
            }

            fn query_dyn_mut_ptr<'__t>(input: &'__t dyn ::meta_tuple::MetaAny) -> Option<Self::OutputPtr<'__t>> {
                Some((#(<#types as ::meta_tuple::MetaQuery>::query_dyn_mut_ptr(input)?,)*))
            }

            fn validate() -> bool {
                #(#type_comparisons)&&*
            }
        }
    }.into()
}
