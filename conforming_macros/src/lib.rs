use darling::*;
use proc_macro2::TokenStream;
use proc_macro_error::*;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Ident, Type};

#[proc_macro_error]
#[proc_macro_derive(ToForm, attributes(input))]
pub fn to_form(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let data = if let Data::Struct(s) = data {
        s
    } else {
        abort_call_site!("ToForm is only applicable to structs")
    };
    let punc = Punctuated::new();
    let fields = match &data.fields {
        syn::Fields::Named(f) => &f.named,
        syn::Fields::Unnamed(f) => &f.unnamed,
        syn::Fields::Unit => &punc,
    }
    .iter()
    .map(|f| match ConformingField::from_field(f) {
        Ok(a) => a,
        Err(err) => {
            if let Some(ident) = &f.ident {
                abort!(
                    ident.span(),
                    "Error with attribute in field {}: {err}",
                    ident
                )
            } else {
                abort_call_site!(err)
            }
        }
    })
    .enumerate()
    .map(|(i, f)| {
        let ty = f.ty;
        let input_type = if let Some(t) = f.input_type {
            quote!(#t)
        } else {
            quote!(<#ty as conforming::FormField>::TYPE)
        };
        let name = f.ident.map_or_else(|| i.to_string(), |i| i.to_string());
        let id = opt(f.id);
        let label = opt(f.label);
        let required = if let Some(t) = f.required {
            quote!(#t)
        } else {
            quote!(<#ty as conforming::FormField>::REQUIRED)
        };
        quote! {
            b = b.with_input(
                #input_type,
                #name,
                #id,
                #label,
                #required,
                vec![]
            );
        }
    })
    .collect::<TokenStream>();

    let output = quote! {
      impl conforming::ToForm for #ident {
          fn to_form() -> conforming::FormBuilder<'static> {
              let mut b = conforming::FormBuilder::new("POST")
                  .with_submit("Send");
              #fields
              b
          }
      }
    };
    output.into()
}

#[derive(Debug, FromField)]
#[darling(attributes(input))]
struct ConformingField {
    ident: Option<Ident>,
    ty: Type,
    input_type: Option<String>,
    id: Option<String>,
    label: Option<String>,
    required: Option<bool>,
}

fn opt<T: ToTokens>(v: Option<T>) -> TokenStream {
    if let Some(v) = v {
        quote!(Some(#v))
    } else {
        quote!(None)
    }
}
