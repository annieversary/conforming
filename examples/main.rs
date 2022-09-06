use conforming::*;
use pretty_assertions::assert_eq;

const FORM_ATTRS: [(&str, Option<&str>); 1] = [("style", Some("background: black; color: white"))];
const NAME_ATTRS: [(&str, Option<&str>); 1] = [("style", Some("background: red"))];
const BUTTON_ATTRS: [(&str, Option<&str>); 1] = [("style", Some("color: blue"))];

#[derive(ToForm)]
#[form(extra_attrs = "FORM_ATTRS", button_attrs = "BUTTON_ATTRS")]
struct MyForm {
    #[input(id = "user_name", extra_attrs = "NAME_ATTRS")]
    name: String,
    #[input(input_type = "email", serializer = "my_serializer")]
    email: Option<String>,
}

fn my_serializer(f: &Option<String>) -> Result<String, ()> {
    let v = if let Some(v) = f {
        v.clone()
    } else {
        "no value".to_string()
    };
    Ok(v)
}

fn main() {
    // generate a form without values
    let html = MyForm::to_form().build();

    assert_eq!(
        html,
        r#"<form action="" method="POST" style="background: black; color: white"><input name="name" type="text" id="user_name" required style="background: red"><input name="email" type="email"><button type="submit" style="color: blue">Send</button></form>"#
    );

    // serialize struct into form
    // useful for edit pages
    let html = MyForm {
        name: "ernesto".to_string(),
        email: None,
    }
    .serialize()
    .unwrap()
    .build();

    assert_eq!(
        html,
        r#"<form action="" method="POST" style="background: black; color: white"><input name="name" type="text" id="user_name" required value="ernesto" style="background: red"><input name="email" type="email" value="no value"><button type="submit" style="color: blue">Send</button></form>"#
    );
}
