//! A crate which defines [`BoundedStructuralOf`] proc-macro.

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

/// Use this derivation to field structs so that array accessor of the struct type can be indexed into a struct of fields.
/// 
/// See `accessor::array::BoundStructual` trait for more details.
#[proc_macro_derive(BoundedStructuralOf)]
pub fn derive_bounded_structural_of(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        vis,
        ident: orig_ident,
        data,
        ..
    } = syn::parse(input).unwrap();

    let bound_ident = Ident::new(&format!("BoundedStructuralOf{}", orig_ident), Span::call_site());

    let fields = match data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("`BoundedStructuralOf` can be derived only for field structs."),
    };
    if let syn::Fields::Named(_) = fields {}
    else {
        // todo: support for tuple structs
        panic!("`BoundedStructuralOf` can be derived only for field structs.");
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
    
    let _field_convert_mut = _field_convert.clone();
    
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

        impl<M, A> accessor::array::BoundedStructural<#orig_ident, M, A> for accessor::array::Generic<#orig_ident, M, A>
        where
            M: accessor::mapper::Mapper,
            A: accessor::marker::Readable,
        {
            type BoundedStructuralType<'a> = #bound_ident<'a, M, accessor::marker::ReadOnly>
            where Self: 'a;

            fn structural_at<'a>(&'a self, i: usize) -> #bound_ident<'a, M, accessor::marker::ReadOnly> {
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

        impl<M, A> accessor::array::BoundedStructuralMut<#orig_ident, M, A> for accessor::array::Generic<#orig_ident, M, A>
        where
            M: accessor::mapper::Mapper,
            A: accessor::marker::Writable,
        {
            type BoundedStructuralType<'a> = #bound_ident<'a, M, A>
            where Self: 'a;

            fn structural_at_mut<'a>(&'a mut self, i: usize) -> #bound_ident<'a, M, A> {
                assert!(i < self.len());
                unsafe {
                    let addr = self.addr(i);
                    #bound_ident {
                        #(#_field_convert_mut)*
                        _lifetime: core::marker::PhantomData
                    }
                }
            }
        }
    };
    tokens.into()
}