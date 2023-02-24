use crate::components::{code::Code, preview::Preview};
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

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Notification"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Notification colors"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Notification light colors"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Notification close"}</ybc::Title>
                <Preview html={preview_4} />
                <Code code={code_4}/>
            </ybc::Section>
        </ybc::Container>
    }
}
