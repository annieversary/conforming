use conforming::*;
use pretty_assertions::assert_eq;

const FORM_ATTRS: [(&str, Option<&str>); 1] = [("style", Some("background: black; color: white"))];
const NAME_ATTRS: [(&str, Option<&str>); 1] = [("style", Some("background: red"))];
const BUTTON_ATTRS: [(&str, Option<&str>); 1] = [("style", Some("color: blue"))];

#[allow(dead_code)]
#[derive(ToForm)]
#[form(extra_attrs = "FORM_ATTRS", button_attrs = "BUTTON_ATTRS")]
struct MyForm {
    #[input(id = "user_name", extra_attrs = "NAME_ATTRS")]
    name: String,
    #[input(
        input_type = "email",
        rename = "user_email",
        no_label,
        serializer = "my_serializer"
    )]
    email: Option<String>,
    #[input(skip)]
    skipped_field: u32,
    #[input(flatten)]
    password: Password,
}

#[derive(ToForm)]
struct Password {
    #[input(input_type = "password", serializer = "pw_serializer")]
    password: String,
    #[input(
        input_type = "password",
        label = "repeat password",
        serializer = "pw_serializer"
    )]
    password_confirmation: String,
}

fn my_serializer(f: &Option<String>) -> Result<String, ()> {
    let v = if let Some(v) = f {
        v.clone()
    } else {
        "no value".to_string()
    };
    Ok(v)
}

fn pw_serializer(_: &String) -> Result<String, ()> {
    Ok("".to_string())
}

fn main() {
    // generate a form without values
    let html = MyForm::to_form().build();

    assert_eq!(
        html,
        r#"<form action="" method="POST" style="background: black; color: white"><label for="user_name">name<input name="name" type="text" id="user_name" required style="background: red"></label><input name="user_email" type="email"><label>password<input name="password" type="password" required></label><label>repeat password<input name="password_confirmation" type="password" required></label><button type="submit" style="color: blue">Send</button></form>"#
    );

    // serialize struct into form
    // useful for edit pages
    let html = MyForm {
        name: "ernesto".to_string(),
        email: None,
        skipped_field: 3,
        password: Password {
            password: "hunter2".to_string(),
            password_confirmation: "hunter2".to_string(),
        },
    }
    .serialize()
    .unwrap()
    .build();

    assert_eq!(
        html,
        r#"<form action="" method="POST" style="background: black; color: white"><label for="user_name">name<input name="name" type="text" id="user_name" required value="ernesto" style="background: red"></label><input name="user_email" type="email" value="no value"><label>password<input name="password" type="password" required value=""></label><label>repeat password<input name="password_confirmation" type="password" required value=""></label><button type="submit" style="color: blue">Send</button></form>"#
    );
}
