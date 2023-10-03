extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(BoundSetGenericOf)]
pub fn derive_bound_set_generic_of(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        vis,
        ident: orig_ident,
        data,
        ..
    } = syn::parse(input).unwrap();

    let bound_ident = Ident::new(&format!("BoundSetGenericOf{}", orig_ident), Span::call_site());

    let fields = match data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("Field can only be derived for field structs"),
    };
    if let syn::Fields::Named(_) = fields {}
    else {
        panic!("Field can only be derived for field structs");
        // todo: support for tuple structs
    }

    let _field_var = fields.iter()
        .map(|field| {
            let vis = field.vis.clone();
            let ident = field.ident.as_ref().unwrap().clone();
            let ty = field.ty.clone();
            quote! {
                #vis #ident: accessor::single::Generic<#ty, accessor::mapper::Identity, A>,
            }
        });
    
    let _field_convert = fields.iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap().clone();
            quote! {
                #ident: accessor::single::Generic::new(addr + accessor::memoffset::offset_of!(#orig_ident, #ident), accessor::mapper::Identity),
            }
        });
    
    let tokens = quote! {
        #[allow(missing_docs)]
        #[allow(missing_debug_implementations)]
        #vis struct #bound_ident<'a, M, A>
        where
            M: accessor::mapper::Mapper,
            A: accessor::marker::AccessorTypeSpecifier,
        {
            #(#_field_var)*
            _lifetime: core::marker::PhantomData<&'a accessor::array::Generic<#orig_ident, M, A>>
        }

        impl<M, A> accessor::array::BoundSetGeneric<#orig_ident, M, A> for accessor::array::Generic<#orig_ident, M, A>
        where
            M: accessor::mapper::Mapper,
            A: accessor::marker::AccessorTypeSpecifier + 'static,
        {
            type BoundSetGenericType<'a> = #bound_ident<'a, M, A>
            where Self: 'a;

            fn set_at<'a>(&'a self, i: usize) -> #bound_ident<'a, M, A> {
                assert!(i < self.len());
                unsafe {
                    let addr = self.addr(i);
                    #bound_ident {
                        #(#_field_convert)*
                        _lifetime: core::marker::PhantomData
                    }
                }
            }
        }
    };
    tokens.into()
}