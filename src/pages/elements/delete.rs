use crate::components::{code::Code, preview::Preview};
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

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Delete"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Delete sizes"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Delete combinations"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
        </ybc::Container>
    }
}
