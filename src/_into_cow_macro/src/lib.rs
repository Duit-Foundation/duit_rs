use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn into_cow(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Генерируем только impl блоки, не меняя структуру
    // Используем crate::core::IntoCow, так как в сгенерированном коде crate будет относиться
    // к крейту, где используется макрос (duit_rs), а не к крейту макроса (into_cow_macro)
    let expanded = quote! {
        #input

        impl<'a> #impl_generics crate::core::IntoCow<'a, #name #ty_generics> for &'a #name #ty_generics #where_clause {
            fn into_cow(self) -> ::std::borrow::Cow<'a, #name #ty_generics> {
                ::std::borrow::Cow::Borrowed(self)
            }
        }

        impl<'a> #impl_generics crate::core::IntoCow<'a, #name #ty_generics> for #name #ty_generics #where_clause {
            fn into_cow(self) -> ::std::borrow::Cow<'a, #name #ty_generics> {
                ::std::borrow::Cow::Owned(self)
            }
        }
    };

    TokenStream::from(expanded)
}