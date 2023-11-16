//! A crate which defines [`BoundedStructuralOf`] proc-macro.

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

/// Use this derivation to field structs so that an accessor of the struct type can be accessed/indexed into a struct of fields.
/// 
/// See `accessor::single::{BoundedStructural, BoundedStructuralMut}` and `accessor::array::{BoundedStructural, BoundededStructuralMut}` traits for more details.
#[proc_macro_derive(BoundedStructuralOf)]
pub fn derive_bounded_structural_of(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        vis,
        ident: orig_ident,
        data,
        ..
    } = syn::parse(input).unwrap();

    let bounded_ident = Ident::new(&format!("BoundedStructuralOf{}", orig_ident), Span::call_site());

    let fields = match data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("`BoundedStructuralOf` can be derived only for field structs."),
    };
    if let syn::Fields::Named(_) = fields {}
    else {
        // todo: support for tuple structs
        panic!("`BoundedStructuralOf` can be derived only for field structs.");
    }

    let field_var = fields.iter()
        .map(|field| {
            let vis = field.vis.clone();
            let ident = field.ident.as_ref().unwrap().clone();
            let ty = field.ty.clone();
            quote! {
                #vis #ident: accessor::single::Generic<#ty, accessor::mapper::Identity, A>,
            }
        });
    
    let field_convert = fields.iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap().clone();
            quote! {
                #ident: accessor::single::Generic::new(addr + accessor::memoffset::offset_of!(#orig_ident, #ident), accessor::mapper::Identity),
            }
        });
    let field_convert_mut = field_convert.clone();
    let field_convert_2 = field_convert.clone();
    let field_convert_2_mut = field_convert.clone();
    
    let tokens = quote! {
        #[allow(missing_docs)]
        #[allow(missing_debug_implementations)]
        #vis struct #bounded_ident<'a, ACC, A>
        where
            A: accessor::marker::AccessorTypeSpecifier,
        {
            #(#field_var)*
            _lifetime: core::marker::PhantomData<&'a ACC>
        }

        impl<M, A> accessor::single::BoundedStructural<#orig_ident, M, A> for accessor::single::Generic<#orig_ident, M, A>
        where
            M: accessor::mapper::Mapper,
            A: accessor::marker::Readable,
        {
            type BoundedStructuralType<'a> = #bounded_ident<'a, Self, accessor::marker::ReadOnly>
            where Self: 'a;

            fn structural<'a>(&'a self) -> #bounded_ident<'a, Self, accessor::marker::ReadOnly> {
                unsafe {
                    let addr = self.addr();
                    #bounded_ident {
                        #(#field_convert)*
                        _lifetime: core::marker::PhantomData
                    }
                }
            }
        }

        impl<M, A> accessor::single::BoundedStructuralMut<#orig_ident, M, A> for accessor::single::Generic<#orig_ident, M, A>
        where
            M: accessor::mapper::Mapper,
            A: accessor::marker::Writable,
        {
            type BoundedStructuralType<'a> = #bounded_ident<'a, Self, A>
            where Self: 'a;

            fn structural_mut<'a>(&'a mut self) -> #bounded_ident<'a, Self, A> {
                unsafe {
                    let addr = self.addr();
                    #bounded_ident {
                        #(#field_convert_mut)*
                        _lifetime: core::marker::PhantomData
                    }
                }
            }
        }

        impl<M, A> accessor::array::BoundedStructural<#orig_ident, M, A> for accessor::array::Generic<#orig_ident, M, A>
        where
            M: accessor::mapper::Mapper,
            A: accessor::marker::Readable,
        {
            type BoundedStructuralType<'a> = #bounded_ident<'a, Self, accessor::marker::ReadOnly>
            where Self: 'a;

            fn structural_at<'a>(&'a self, i: usize) -> #bounded_ident<'a, Self, accessor::marker::ReadOnly> {
                assert!(i < self.len());
                unsafe {
                    let addr = self.addr(i);
                    #bounded_ident {
                        #(#field_convert_2)*
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
            type BoundedStructuralType<'a> = #bounded_ident<'a, Self, A>
            where Self: 'a;

            fn structural_at_mut<'a>(&'a mut self, i: usize) -> #bounded_ident<'a, Self, A> {
                assert!(i < self.len());
                unsafe {
                    let addr = self.addr(i);
                    #bounded_ident {
                        #(#field_convert_2_mut)*
                        _lifetime: core::marker::PhantomData
                    }
                }
            }
        }
    };
    tokens.into()
}