use conforming::*;

#[derive(ToForm)]
struct MyForm {
    #[input(id = "user_name")]
    name: String,
    #[input(input_type = "email")]
    email: Option<String>,
}

fn main() {
    let html = MyForm::to_form().build();

    assert_eq!(
        html,
        r#"<form action="" method="POST"><input name="name" type="text" id="user_name" required><input name="email" type="email"><button type="submit">Send</button></form>"#
    )
}
