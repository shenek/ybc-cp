use crate::components::description;
use std::rc::Rc;
use yew::prelude::*;

#[function_component(Delete)]
pub fn delete() -> Html {
    let code_1 = r#"html! {
        <ybc::Delete></ybc::Delete>
}"#;
    let preview_1 = html! {
        <ybc::Delete></ybc::Delete>
    };

    let code_2 = r#"html! {
    <ybc::Delete classes={classes!("is-small")}></ybc::Delete>
    <ybc::Delete></ybc::Delete>
    <ybc::Delete classes={classes!("is-medium")}></ybc::Delete>
    <ybc::Delete classes={classes!("is-large")}></ybc::Delete>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Delete classes={classes!("is-small")}></ybc::Delete>
        <ybc::Delete></ybc::Delete>
        <ybc::Delete classes={classes!("is-medium")}></ybc::Delete>
        <ybc::Delete classes={classes!("is-large")}></ybc::Delete>
        </>
    };

    let code_3 = r#"html! {
    <ybc::Tag classes={classes!("is-success")}>
        {"Delete in tag"}
        <ybc::Delete classes={classes!("is-small")}></ybc::Delete>
    </ybc::Tag>
    <ybc::Notification classes={classes!("is-danger")}>
        <ybc::Delete></ybc::Delete>
        {"Delete in notification"}
    </ybc::Notification>
    <ybc::Message classes={classes!("is-info")}>
        <ybc::MessageHeader>
            {"Delete in message"}
            <ybc::Delete></ybc::Delete>
        </ybc::MessageHeader>
        <ybc::MessageBody>
            {"Text of the message can be quite long."}
        </ybc::MessageBody>
    </ybc::Message>
}"#;
    let preview_3 = html! {
        <>
        <ybc::Tag classes={classes!("is-success", "mb-2")}>
            {"Delete in tag"}
            <ybc::Delete classes={classes!("is-small")}></ybc::Delete>
        </ybc::Tag>
        <ybc::Notification classes={classes!("is-danger")}>
            <ybc::Delete></ybc::Delete>
            {"Delete in notification"}
        </ybc::Notification>
        <ybc::Message classes={classes!("is-info")}>
            <ybc::MessageHeader>
                {"Delete in message"}
                <ybc::Delete></ybc::Delete>
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Text of the message can be quite long."}
            </ybc::MessageBody>
        </ybc::Message>
        </>
    };

    let descriptions = vec![
        description::Item {
            title: "Delete".to_owned(),
            id: "delete".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Delete sizes".to_owned(),
            id: "delete-sizes".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Delete combinations".to_owned(),
            id: "delete-combinations".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
    ];

    html! {
        <description::Description items={descriptions} />
    }
}
