use darling::*;
use proc_macro2::TokenStream;
use proc_macro_error::*;
use quote::quote;
use syn::{parse_macro_input, Ident, Type, TypePath};

#[proc_macro_error]
#[proc_macro_derive(ToForm, attributes(input, form))]
pub fn to_form(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let di = parse_macro_input!(input);
    let form = Conforming::from_derive_input(&di).unwrap();

    let fields = form.data.take_struct().unwrap();

    let to_form = fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            let input_type = if let Some(t) = &f.input_type {
                quote!(#t)
            } else {
                quote!(<#ty as conforming::FormField>::TYPE)
            };
            let name_str = f.ident.as_ref().unwrap().to_string();
            let id = opt(f.id.as_ref());
            let label = opt(f.label.as_ref());
            let required = if let Some(t) = f.required {
                quote!(#t)
            } else {
                quote!(<#ty as conforming::FormField>::REQUIRED)
            };
            let attrs = if let Some(a) = &f.extra_attrs {
                quote!(&#a)
            } else {
                quote!(&[])
            };
            quote! {
                b = b.with_input(
                    #input_type,
                    #name_str,
                    #id,
                    #label,
                    #required,
                    #attrs,
                    None,
                );
            }
        })
        .collect::<TokenStream>();
    let ser = fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            let input_type = if let Some(t) = &f.input_type {
                quote!(#t)
            } else {
                quote!(<#ty as conforming::FormField>::TYPE)
            };
            let name = f.ident.as_ref().unwrap();
            let name_str = f.ident.as_ref().unwrap().to_string();
            let id = opt(f.id.as_ref());
            let label = opt(f.label.as_ref());
            let required = if let Some(t) = f.required {
                quote!(#t)
            } else {
                quote!(<#ty as conforming::FormField>::REQUIRED)
            };
            let ser = if let Some(ser) = &f.serializer {
                quote!(#ser)
            } else {
                quote!(<#ty as conforming::FormField>::SERIALIZER)
            };
            let attrs = if let Some(a) = &f.extra_attrs {
                quote!(&#a)
            } else {
                quote!(&[])
            };
            quote! {
                b = b.with_input(
                    #input_type,
                    #name_str,
                    #id,
                    #label,
                    #required,
                    #attrs,
                    Some(#ser(&self.#name).map_err(|_| FormSerializeError::FieldSerializationError(#name_str))?),
                );
            }
        })
        .collect::<TokenStream>();

    let ident = form.ident;

    let form_attrs = form.extra_attrs.map(|p| {
        quote! {
            b = b.with_form_attrs(&#p);
        }
    });
    let button_attrs = form.button_attrs.map(|p| {
        quote! {
            b = b.with_button_attrs(&#p);
        }
    });

    let output = quote! {
      impl conforming::ToForm for #ident {
          fn to_form() -> conforming::FormBuilder<'static> {
              let mut b = conforming::FormBuilder::new("POST")
                  .with_submit("Send");
              #form_attrs
              #button_attrs
              #to_form
              b
          }

          fn serialize(&self) -> Result<FormBuilder<'static>, FormSerializeError> {
              let mut b = conforming::FormBuilder::new("POST")
                  .with_submit("Send");
              #form_attrs
              #button_attrs
              #ser
              Ok(b)
          }
      }
    };
    output.into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(input, form), supports(struct_named))]
struct Conforming {
    ident: Ident,
    data: ast::Data<util::Ignored, ConformingField>,

    extra_attrs: Option<TypePath>,
    button_attrs: Option<TypePath>,
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
    serializer: Option<TypePath>,
    extra_attrs: Option<TypePath>,
}

fn opt<T: ToTokens>(v: Option<T>) -> TokenStream {
    if let Some(v) = v {
        quote!(Some(#v))
    } else {
        quote!(None)
    }
}
