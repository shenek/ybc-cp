use crate::components::{code::Code, preview::Preview};
use yew::prelude::*;

#[function_component(Message)]
pub fn message() -> Html {
    let code_1 = r#"html! {
    <ybc::Message>
        <ybc::MessageHeader>
            {"Simple message"}
            <ybc::Delete />
        </ybc::MessageHeader>
        <ybc::MessageBody>
            {"Some body content"}
        </ybc::MessageBody>
    </ybc::Message>
}"#;
    let preview_1 = html! {
        <ybc::Message>
            <ybc::MessageHeader>
                {"Simple message"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some body content"}
            </ybc::MessageBody>
        </ybc::Message>
    };

    let code_2 = r#"html! {
        <ybc::Message classes={classes!("is-dark")}>
            <ybc::MessageHeader>
                {"Dark"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some dark body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-primary")}>
            <ybc::MessageHeader>
                {"Primary"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some primary body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-link")}>
            <ybc::MessageHeader>
                {"Link"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some link body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-info")}>
            <ybc::MessageHeader>
                {"Info"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some info body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-success")}>
            <ybc::MessageHeader>
                {"Success"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some success body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-warning")}>
            <ybc::MessageHeader>
                {"Warning"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some warning body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-danger")}>
            <ybc::MessageHeader>
                {"Danger"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some danger body content"}
            </ybc::MessageBody>
        </ybc::Message>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Message classes={classes!("is-dark")}>
            <ybc::MessageHeader>
                {"Dark"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some dark body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-primary")}>
            <ybc::MessageHeader>
                {"Primary"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some primary body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-link")}>
            <ybc::MessageHeader>
                {"Link"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some link body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-info")}>
            <ybc::MessageHeader>
                {"Info"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some info body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-success")}>
            <ybc::MessageHeader>
                {"Success"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some success body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-warning")}>
            <ybc::MessageHeader>
                {"Warning"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some warning body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-danger")}>
            <ybc::MessageHeader>
                {"Danger"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"Some danger body content"}
            </ybc::MessageBody>
        </ybc::Message>
        </>
    };

    let code_3 = r#"html! {
    <ybc::Message>
        <ybc::MessageBody>
            {"Some only body content"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-dark")}>
        <ybc::MessageBody>
            {"Some dark only body content"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-primary")}>
        <ybc::MessageBody>
            {"Some primary only body content"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-link")}>
        <ybc::MessageBody>
            {"Some link only body content"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-info")}>
        <ybc::MessageBody>
            {"Some info only body content"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-success")}>
        <ybc::MessageBody>
            {"Some success only body content"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-warning")}>
        <ybc::MessageBody>
            {"Some warning only body content"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-danger")}>
        <ybc::MessageBody>
            {"Some danger only body content"}
        </ybc::MessageBody>
    </ybc::Message>
}"#;
    let preview_3 = html! {
        <>
        <ybc::Message>
            <ybc::MessageBody>
                {"Some only body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-dark")}>
            <ybc::MessageBody>
                {"Some dark only body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-primary")}>
            <ybc::MessageBody>
                {"Some primary only body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-link")}>
            <ybc::MessageBody>
                {"Some link only body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-info")}>
            <ybc::MessageBody>
                {"Some info only body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-success")}>
            <ybc::MessageBody>
                {"Some success only body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-warning")}>
            <ybc::MessageBody>
                {"Some warning only body content"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-danger")}>
            <ybc::MessageBody>
                {"Some danger only body content"}
            </ybc::MessageBody>
        </ybc::Message>
        </>
    };

    let code_4 = r#"html! {
    <ybc::Message classes={classes!("is-small")}>
        <ybc::MessageHeader>
            {"Small sized"}
            <ybc::Delete />
        </ybc::MessageHeader>
        <ybc::MessageBody>
            {"This is message with small size"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message>
        <ybc::MessageHeader>
            {"Normal sized"}
            <ybc::Delete />
        </ybc::MessageHeader>
        <ybc::MessageBody>
            {"This is message with normal size"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-medium")}>
        <ybc::MessageHeader>
            {"Medium sized"}
            <ybc::Delete />
        </ybc::MessageHeader>
        <ybc::MessageBody>
            {"This is message with medium size"}
        </ybc::MessageBody>
    </ybc::Message>
    <ybc::Message classes={classes!("is-large")}>
        <ybc::MessageHeader>
            {"Large sized"}
            <ybc::Delete />
        </ybc::MessageHeader>
        <ybc::MessageBody>
            {"This is message with large size"}
        </ybc::MessageBody>
    </ybc::Message>
}"#;
    let preview_4 = html! {
        <>
        <ybc::Message classes={classes!("is-small")}>
            <ybc::MessageHeader>
                {"Small sized"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"This is message with small size"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message>
            <ybc::MessageHeader>
                {"Normal sized"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"This is message with normal size"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-medium")}>
            <ybc::MessageHeader>
                {"Medium sized"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"This is message with medium size"}
            </ybc::MessageBody>
        </ybc::Message>
        <ybc::Message classes={classes!("is-large")}>
            <ybc::MessageHeader>
                {"Large sized"}
                <ybc::Delete />
            </ybc::MessageHeader>
            <ybc::MessageBody>
                {"This is message with large size"}
            </ybc::MessageBody>
        </ybc::Message>
        </>
    };

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Message"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Message colors"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Message body only"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Message sizes"}</ybc::Title>
                <Preview html={preview_4} />
                <Code code={code_4}/>
            </ybc::Section>
        </ybc::Container>
    }
}
