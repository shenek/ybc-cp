use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(Notification)]
pub fn notification() -> Html {
    let code_1 = r#"html! {
    <ybc::Notification>
        {"This is Notification!"}
    </ybc::Notification>
}"#;
    let preview_1 = html! {
        <ybc::Notification>
            {"This is Notification!"}
        </ybc::Notification>
    };

    let code_2 = r#"html! {
    <>
    <ybc::Notification classes={classes!("is-primary")}>
        {"This is primary Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-link")}>
        {"This is link Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-info")}>
        {"This is info Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-success")}>
        {"This is success Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-warning")}>
        {"This is warning Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-danger")}>
        {"This is danger Notification!"}
    </ybc::Notification>
    </>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Notification classes={classes!("is-primary")}>
            {"This is primary Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-link")}>
            {"This is link Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-info")}>
            {"This is info Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-success")}>
            {"This is success Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-warning")}>
            {"This is warning Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-danger")}>
            {"This is danger Notification!"}
        </ybc::Notification>
        </>
    };

    let code_3 = r#"html! {
    <>
    <ybc::Notification classes={classes!("is-primary", "is-light")}>
        {"This is light primary Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-link", "is-light")}>
        {"This is light link Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-info", "is-light")}>
        {"This is light info Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-success", "is-light")}>
        {"This is light success Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-warning", "is-light")}>
        {"This is light warning Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-danger", "is-light")}>
        {"This is light danger Notification!"}
    </ybc::Notification>
    </>
}"#;
    let preview_3 = html! {
        <>
        <ybc::Notification classes={classes!("is-primary", "is-light")}>
            {"This is light primary Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-link", "is-light")}>
            {"This is light link Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-info", "is-light")}>
            {"This is light info Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-success", "is-light")}>
            {"This is light success Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-warning", "is-light")}>
            {"This is light warning Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-danger", "is-light")}>
            {"This is light danger Notification!"}
        </ybc::Notification>
        </>
    };

    let code_4 = r#"let notification_active = use_state_eq(|| true);
let close_notification_cb = {
    let notification_active = notification_active.clone();
    Callback::from(move |_| {
        notification_active.set(false);
    })
    
};

html! {
    {
        if *notification_active {
            html! {
                <ybc::Notification>
                    <ybc::Delete tag={"button"} onclick={close_notification_cb} />
                    {"This is closable Notification!"}
                </ybc::Notification>
            }
        } else {
            html! {}
        }
    }
}"#;
    let notification_active = use_state_eq(|| true);
    let close_notification_cb = {
        let notification_active = notification_active.clone();
        Callback::from(move |_| {
            notification_active.set(false);
        })
    };
    let preview_4 = html! {
        {
            if *notification_active {
                html! {
                    <ybc::Notification>
                        <ybc::Delete tag={"button"} onclick={close_notification_cb} />
                        {"This is closable Notification!"}
                    </ybc::Notification>
                }
            } else {
                html! {}
            }
        }
    };

    let descriptions = vec![
        description::Item {
            title: "Notification".to_owned(),
            id: "notification".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Notification colors".to_owned(),
            id: "notification-colors".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Notification light colors".to_owned(),
            id: "notification-light-colors".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
        description::Item {
            title: "Notification close".to_owned(),
            id: "notification-close".to_owned(),
            code: code_4.to_owned(),
            html: Rc::new(preview_4),
        },
    ];

    let api: Vec<description::Api> = vec![(
        "<ybc::Notification>",
        "https://docs.rs/ybc/latest/ybc/struct.NotificationProps.html",
    )
        .into()];

    html! {
        <description::Description items={descriptions} {api} />
    }
}
