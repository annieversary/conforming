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

    let to_form = to_form_fields(fields.iter());
    let ser = ser_fields(fields.iter());

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

          fn serialize(&self) -> Result<conforming::FormBuilder<'static>, conforming::FormSerializeError> {
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

fn to_form_fields<'a>(f: impl Iterator<Item = &'a ConformingField>) -> TokenStream {
    f.filter(|f| !f.skip)
        .map(|f| {
            if f.flatten {
                flat_field(f, false)
            } else {
                field(f, false)
            }
        })
        .collect::<TokenStream>()
}

fn ser_fields<'a>(f: impl Iterator<Item = &'a ConformingField>) -> TokenStream {
    f.filter(|f| !f.skip)
        .map(|f| {
            if f.flatten {
                flat_field(f, true)
            } else {
                field(f, true)
            }
        })
        .collect::<TokenStream>()
}

fn field(f: &ConformingField, serialize: bool) -> TokenStream {
    let ty = &f.ty;
    let input_type = if let Some(t) = &f.input_type {
        quote!(#t)
    } else {
        quote!(<#ty as conforming::FormField>::TYPE)
    };
    let name = f.ident.as_ref().unwrap();
    let name_str = name.to_string();
    let id = opt(f.id.as_ref());

    let no_label = f.no_label;
    let label = opt((!no_label).then(|| f.label.clone().or_else(|| Some(name_str.clone()))));
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

    let value = if serialize {
        let ser = if let Some(ser) = &f.serializer {
            quote!(#ser)
        } else {
            quote!(<#ty as conforming::FormField>::SERIALIZER)
        };
        quote! {
            Some(#ser(&self.#name).map_err(|_| conforming::FormSerializeError::FieldSerializationError(#name_str))?)
        }
    } else {
        quote!(None)
    };
    quote! {
        b = b.with_input(
            #input_type,
            #name_str,
            #id,
            #label,
            #required,
            #attrs,
            #value,
        );
    }
}

fn flat_field(f: &ConformingField, serialize: bool) -> TokenStream {
    let fun = if serialize {
        let name = f.ident.as_ref().unwrap();
        quote!(self.#name.serialize().unwrap())
    } else {
        let ty = &f.ty;
        quote!(#ty::to_form())
    };
    quote! {
        {
            let form = #fun;
            for field in form.fields {
                b = b.with_input(
                    field.input_type,
                    field.name,
                    field.id,
                    field.label,
                    field.required,
                    field.attributes,
                    field.value,
                );
            }
        }
    }
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
    extra_attrs: Option<TypePath>,
    serializer: Option<TypePath>,
    #[darling(default)]
    skip: bool,
    #[darling(default)]
    no_label: bool,
    #[darling(default)]
    flatten: bool,
}

fn opt<T: ToTokens>(v: Option<T>) -> TokenStream {
    if let Some(v) = v {
        quote!(Some(#v))
    } else {
        quote!(None)
    }
}
