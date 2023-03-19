use crate::components::{code::Code, preview::Preview};
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let code_1 = r#"html! {
    <ybc::Footer>
        <ybc::Content classes={classes!("has-text-centered")}>
            <strong>{"YBC - C&P"}</strong>{" on "}<a href={"https://github.com/shenek/ybc-cp"}>{"github"}</a>{"."}
        </ybc::Content>
    </ybc::Footer>
}"#;
    let preview_1 = html! {
        <ybc::Footer>
            <ybc::Content classes={classes!("has-text-centered")}>
                <strong>{"YBC - C&P"}</strong>{" on "}<a href={"https://github.com/shenek/ybc-cp"}>{"github"}</a>{"."}
            </ybc::Content>
        </ybc::Footer>
    };
    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Footer"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
        </ybc::Container>
    }
}
