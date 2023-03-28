use std::rc::Rc;

use crate::components::{description};
use yew::prelude::*;

#[function_component(General)]
pub fn general() -> Html {
    let code_1 = r#"
let name = use_state_eq(|| String::new());
let name_updated_cb = {
    let name = name.clone();
    Callback::from(move |new_name| name.set(new_name))
};
let username = use_state_eq(|| "user1".to_string());
let username_updated_cb = {
    let username = username.clone();
    Callback::from(move |new_username| username.set(new_username))
};
let email = use_state_eq(|| "hello@".to_string());
let email_updated_cb = {
    let email = email.clone();
    Callback::from(move |new_email| email.set(new_email))
};
let subject = use_state_eq(|| "s2".to_string());
let subject_updated_cb = {
    let subject = subject.clone();
    Callback::from(move |new_subject| subject.set(new_subject))
};
let message = use_state_eq(|| "".to_string());
let message_updated_cb = {
    let message = message.clone();
    Callback::from(move |new_message| message.set(new_message))
};
let secure = use_state_eq(|| false);
let secure_updated_cb = {
    let secure = secure.clone();
    Callback::from(move |new_secure| secure.set(new_secure))
};
let provider = use_state_eq(|| None);
let provider_updated_cb = {
    let provider = provider.clone();
    Callback::from(move |new_provider: String| provider.set(Some(new_provider.to_string())))
};

html! {
    <ybc::Field label={format!("Name: {}", (*name))}>
        <ybc::Control>
            <ybc::Input
              placeholder={"Text input"}
              r#type={ybc::InputType::Text}
              name={"form_name"}
              value={(*name).clone()}
              update={name_updated_cb}
            >
            </ybc::Input>
        </ybc::Control>
    </ybc::Field>
    <ybc::Field label={"Username:"}>
        <ybc::Control classes={classes!("has-icons-left", "has-icons-right")}>
            <ybc::Input
              placeholder={"Text input"}
              r#type={ybc::InputType::Text}
              name={"form_username"}
              value={(*username).clone()}
              update={username_updated_cb}
              classes={classes!("is-success")}
            >
            </ybc::Input>
            <ybc::Icon alignment={ybc::Alignment::Left}>
                <i class={classes!("fas", "fa-user")}></i>
            </ybc::Icon>
            <ybc::Icon alignment={ybc::Alignment::Right} size={ybc::Size::Small}>
                <i class={classes!("fas", "fa-check")}></i>
            </ybc::Icon>
        </ybc::Control>
        <p class={classes!("help", "is-success")}>{format!("Username '{}' is available", *username)}</p>
    </ybc::Field>
    <ybc::Field label={"Email:"}>
        <ybc::Control classes={classes!("has-icons-left", "has-icons-right")}>
            <ybc::Input
              placeholder={"Email input"}
              r#type={ybc::InputType::Email}
              name={"form_email"}
              value={(*email).clone()}
              update={email_updated_cb}
              classes={classes!("is-danger")}
            >
            </ybc::Input>
            <ybc::Icon alignment={ybc::Alignment::Left}>
                <i class={classes!("fas", "fa-envelope")}></i>
            </ybc::Icon>
            <ybc::Icon alignment={ybc::Alignment::Right} size={ybc::Size::Small}>
                <i class={classes!("fas", "fa-exclamation-triangle")}></i>
            </ybc::Icon>
        </ybc::Control>
        <p class={classes!("help", "is-danger")}>{format!("Email '{}' is invalid", *email)}</p>
    </ybc::Field>
    <ybc::Field label={format!("Subject: {}", *subject)}>
        <ybc::Control>
            <ybc::Select
              name={"form_subject"}
              update={subject_updated_cb}
              value={(*subject).clone()}
            >
                <option value={"s1"}>{"Subject 1"}</option>
                <option value={"s2"}>{"Subject 2"}</option>
            </ybc::Select>
        </ybc::Control>
    </ybc::Field>
    <ybc::Field label={format!("Message: {}", (*message).len())}>
        <ybc::Control>
            <ybc::TextArea
              name={"form_message"}
              update={message_updated_cb}
              value={(*message).clone()}
              placeholder={"TextArea"}
            >
            </ybc::TextArea>
        </ybc::Control>
    </ybc::Field>
    <ybc::Field label={format!("Secure: {}", *secure)}>
        <ybc::Control>
            <ybc::Checkbox
              name={"form_secure"}
              update={secure_updated_cb}
              checked={*secure}
            >
                <span class={classes!("ml-1")}>{"Send in more secure way"}</span>
            </ybc::Checkbox>
        </ybc::Control>
    </ybc::Field>
    <ybc::Field label={format!("Provider: {}", (*provider).as_ref().map(|e| e.to_string()).unwrap_or_default())}>
        <ybc::Control>
            <ybc::Radio
              name={"form_provider"}
              update={provider_updated_cb.clone()}
              checked_value={(*provider).clone()}
              value={"google"}
            >
                <span class={classes!("ml-1")}>{"Google"}</span>
            </ybc::Radio>
            <ybc::Radio
              name={"form_provider"}
              update={provider_updated_cb}
              checked_value={(*provider).clone()}
              value={"yahoo"}
            >
                <span class={classes!("ml-1")}>{"Yahoo"}</span>
            </ybc::Radio>
        </ybc::Control>
    </ybc::Field>
    <ybc::Field grouped={true}>
        <ybc::Control>
            <ybc::Button classes={classes!("is-link")}>{"Submit"}</ybc::Button>
        </ybc::Control>
        <ybc::Control>
            <ybc::Button classes={classes!("is-link", "is-light")}>{"Cancel"}</ybc::Button>
        </ybc::Control>
    </ybc::Field>
}"#;

    let name = use_state_eq(|| String::new());
    let name_updated_cb = {
        let name = name.clone();
        Callback::from(move |new_name| name.set(new_name))
    };
    let username = use_state_eq(|| "user1".to_string());
    let username_updated_cb = {
        let username = username.clone();
        Callback::from(move |new_username| username.set(new_username))
    };
    let email = use_state_eq(|| "hello@".to_string());
    let email_updated_cb = {
        let email = email.clone();
        Callback::from(move |new_email| email.set(new_email))
    };
    let subject = use_state_eq(|| "s2".to_string());
    let subject_updated_cb = {
        let subject = subject.clone();
        Callback::from(move |new_subject| subject.set(new_subject))
    };
    let message = use_state_eq(|| "".to_string());
    let message_updated_cb = {
        let message = message.clone();
        Callback::from(move |new_message| message.set(new_message))
    };
    let secure = use_state_eq(|| false);
    let secure_updated_cb = {
        let secure = secure.clone();
        Callback::from(move |new_secure| secure.set(new_secure))
    };
    let provider = use_state_eq(|| None);
    let provider_updated_cb = {
        let provider = provider.clone();
        Callback::from(move |new_provider: String| provider.set(Some(new_provider.to_string())))
    };
    let preview_1 = html! {
        <>
            <ybc::Field label={format!("Name: {}", (*name))}>
                <ybc::Control>
                    <ybc::Input
                      placeholder={"Text input"}
                      r#type={ybc::InputType::Text}
                      name={"form_name"}
                      value={(*name).clone()}
                      update={name_updated_cb}
                    >
                    </ybc::Input>
                </ybc::Control>
            </ybc::Field>
            <ybc::Field label={"Username:"}>
                <ybc::Control classes={classes!("has-icons-left", "has-icons-right")}>
                    <ybc::Input
                      placeholder={"Text input"}
                      r#type={ybc::InputType::Text}
                      name={"form_username"}
                      value={(*username).clone()}
                      update={username_updated_cb}
                      classes={classes!("is-success")}
                    >
                    </ybc::Input>
                    <ybc::Icon alignment={ybc::Alignment::Left}>
                        <i class={classes!("fas", "fa-user")}></i>
                    </ybc::Icon>
                    <ybc::Icon alignment={ybc::Alignment::Right} size={ybc::Size::Small}>
                        <i class={classes!("fas", "fa-check")}></i>
                    </ybc::Icon>
                </ybc::Control>
                <p class={classes!("help", "is-success")}>{format!("Username '{}' is available", *username)}</p>
            </ybc::Field>
            <ybc::Field label={"Email:"}>
                <ybc::Control classes={classes!("has-icons-left", "has-icons-right")}>
                    <ybc::Input
                      placeholder={"Email input"}
                      r#type={ybc::InputType::Email}
                      name={"form_email"}
                      value={(*email).clone()}
                      update={email_updated_cb}
                      classes={classes!("is-danger")}
                    >
                    </ybc::Input>
                    <ybc::Icon alignment={ybc::Alignment::Left}>
                        <i class={classes!("fas", "fa-envelope")}></i>
                    </ybc::Icon>
                    <ybc::Icon alignment={ybc::Alignment::Right} size={ybc::Size::Small}>
                        <i class={classes!("fas", "fa-exclamation-triangle")}></i>
                    </ybc::Icon>
                </ybc::Control>
                <p class={classes!("help", "is-danger")}>{format!("Email '{}' is invalid", *email)}</p>
            </ybc::Field>
            <ybc::Field label={format!("Subject: {}", *subject)}>
                <ybc::Control>
                    <ybc::Select
                      name={"form_subject"}
                      update={subject_updated_cb}
                      value={(*subject).clone()}
                    >
                        <option value={"s1"}>{"Subject 1"}</option>
                        <option value={"s2"}>{"Subject 2"}</option>
                    </ybc::Select>
                </ybc::Control>
            </ybc::Field>
            <ybc::Field label={format!("Message: {}", (*message).len())}>
                <ybc::Control>
                    <ybc::TextArea
                      name={"form_message"}
                      update={message_updated_cb}
                      value={(*message).clone()}
                      placeholder={"TextArea"}
                    >
                    </ybc::TextArea>
                </ybc::Control>
            </ybc::Field>
            <ybc::Field label={format!("Secure: {}", *secure)}>
                <ybc::Control>
                    <ybc::Checkbox
                      name={"form_secure"}
                      update={secure_updated_cb}
                      checked={*secure}
                    >
                        <span class={classes!("ml-1")}>{"Send in more secure way"}</span>
                    </ybc::Checkbox>
                </ybc::Control>
            </ybc::Field>
            <ybc::Field label={format!("Provider: {}", (*provider).as_ref().map(|e| e.to_string()).unwrap_or_default())}>
                <ybc::Control>
                    <ybc::Radio
                      name={"form_provider"}
                      update={provider_updated_cb.clone()}
                      checked_value={(*provider).clone()}
                      value={"google"}
                    >
                        <span class={classes!("ml-1")}>{"Google"}</span>
                    </ybc::Radio>
                    <ybc::Radio
                      name={"form_provider"}
                      update={provider_updated_cb}
                      checked_value={(*provider).clone()}
                      value={"yahoo"}
                    >
                        <span class={classes!("ml-1")}>{"Yahoo"}</span>
                    </ybc::Radio>
                </ybc::Control>
            </ybc::Field>
            <ybc::Field grouped={true}>
                <ybc::Control>
                    <ybc::Button classes={classes!("is-link")}>{"Submit"}</ybc::Button>
                </ybc::Control>
                <ybc::Control>
                    <ybc::Button classes={classes!("is-link", "is-light")}>{"Cancel"}</ybc::Button>
                </ybc::Control>
            </ybc::Field>
        </>
    };

    let descriptions = vec![
        description::Item {
            title: "Complete form".to_string(),
            id: "form-complete".to_string(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        }
    ];

    let api: Vec<description::Api> = vec![
        (
            "<ybc::Field>",
            "https://docs.rs/ybc/latest/ybc/struct.FieldProps.html",
        )
            .into(),
        (
            "<ybc::Control>",
            "https://docs.rs/ybc/latest/ybc/struct.ControlProps.html",
        )
            .into(),
        (
            "<ybc::Input>",
            "https://docs.rs/ybc/latest/ybc/struct.InputProps.html",
        )
            .into(),
        (
            "<ybc::Select>",
            "https://docs.rs/ybc/latest/ybc/struct.SelectProps.html",
        )
            .into(),
        (
            "<ybc::TextArea>",
            "https://docs.rs/ybc/latest/ybc/struct.TextAreaProps.html",
        )
            .into(),
        (
            "<ybc::Checkbox>",
            "https://docs.rs/ybc/latest/ybc/struct.CheckboxProps.html",
        )
            .into(),
        (
            "<ybc::Radio>",
            "https://docs.rs/ybc/latest/ybc/struct.RadioProps.html",
        )
            .into(),
        (
            "<ybc::Button>",
            "https://docs.rs/ybc/latest/ybc/struct.ButtonProps.html",
        )
            .into(),
    ];

    html! {
        <description::Description items={descriptions} {api} />
    }

}
