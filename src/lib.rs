pub use conforming_macros::ToForm;

mod form_field;
pub mod helpers;
pub use form_field::*;

pub trait ToForm {
    fn to_form() -> FormBuilder<'static>;
    fn serialize(&self) -> Result<FormBuilder<'static>, FormSerializeError>;
}

pub struct FormBuilder<'a> {
    method: &'a str,
    action: Option<&'a str>,
    submit: Option<&'a str>,

    form_attrs: Option<&'a [(&'a str, Option<&'a str>)]>,
    button_attrs: Option<&'a [(&'a str, Option<&'a str>)]>,

    pub fields: Vec<Field<'a>>,
}
impl<'a> FormBuilder<'a> {
    pub fn new(method: &'a str) -> Self {
        Self {
            method,
            action: None,
            submit: None,
            fields: vec![],
            form_attrs: None,
            button_attrs: None,
        }
    }

    pub fn with_method(mut self, method: &'a str) -> Self {
        self.method = method;
        self
    }

    pub fn with_action(mut self, action: &'a str) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_submit(mut self, submit: &'a str) -> Self {
        self.submit = Some(submit);
        self
    }

    pub fn with_form_attrs(mut self, form_attrs: &'a [(&'a str, Option<&'a str>)]) -> Self {
        self.form_attrs = Some(form_attrs);
        self
    }

    pub fn with_button_attrs(mut self, button_attrs: &'a [(&'a str, Option<&'a str>)]) -> Self {
        self.button_attrs = Some(button_attrs);
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_input(
        mut self,
        input_type: &'a str,
        name: &'a str,
        id: Option<&'a str>,
        label: Option<&'a str>,
        required: bool,
        attributes: &'a [(&'a str, Option<&'a str>)],
        value: Option<String>,
    ) -> Self {
        self.fields.push(Field {
            input_type,
            id,
            name,
            label,
            required,
            attributes,
            value,
        });
        self
    }

    pub fn build(self) -> String {
        let mut o = format!(
            r#"<form action="{}" method="{}""#,
            self.action.unwrap_or_default(),
            self.method
        );

        macro_rules! append {
            ( $($s:expr),* $(,)? ) => {
                $(o.push_str($s);)*
            };
        }

        if let Some(a) = self.form_attrs {
            for (name, value) in a {
                append!(" ", name);
                if let Some(value) = value {
                    append!("=\"", value, "\"");
                }
            }
        }
        append!(">");

        for field in self.fields {
            if let Some(label) = field.label {
                append!("<label");
                if let Some(id) = field.id {
                    append!(" for=\"", id, "\"");
                }
                append!(">", label);
            }

            append!(
                "<input name=\"",
                field.name,
                "\" type=\"",
                field.input_type,
                "\""
            );
            if let Some(id) = field.id {
                append!(" id=\"", id, "\"");
            }
            if field.required {
                append!(" required");
            }
            if let Some(val) = &field.value {
                append!(" value=\"", val, "\"");
            }
            for (name, value) in field.attributes {
                append!(" ", name);
                if let Some(value) = value {
                    append!("=\"", value, "\"");
                }
            }
            append!(">");

            if field.label.is_some() {
                append!("</label>");
            }
        }

        if let Some(submit) = self.submit {
            append!("<button type=\"submit\"");
            if let Some(a) = self.button_attrs {
                for (name, value) in a {
                    append!(" ", name);
                    if let Some(value) = value {
                        append!("=\"", value, "\"");
                    }
                }
            }
            append!(">", submit, "</button>");
        }

        append!("</form>");

        o
    }
}

// TODO allow checkboxes and radio buttons

pub struct Field<'a> {
    pub input_type: &'a str,
    pub name: &'a str,
    pub id: Option<&'a str>,
    pub label: Option<&'a str>,
    pub required: bool,
    pub attributes: &'a [(&'a str, Option<&'a str>)],
    pub value: Option<String>,
}

#[derive(Debug)]
pub enum FormSerializeError {
    FieldSerializationError(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_name() {
        let html = FormBuilder::new("POST")
            .with_action("/")
            .with_submit("go")
            .with_form_attrs(&[("a", Some("b"))])
            .with_button_attrs(&[("c", None)])
            .with_input("string", "name", Some("name"), None, false, &[], None)
            .with_input(
                "email",
                "email",
                Some("email"),
                None,
                false,
                &[],
                "something@example.com".to_string().into(),
            )
            .build();

        assert_eq!(
            html,
            r#"<form action="/" method="POST" a="b"><input name="name" type="string" id="name"><input name="email" type="email" id="email" value="something@example.com"><button type="submit" c>go</button></form>"#
        )
    }
}
