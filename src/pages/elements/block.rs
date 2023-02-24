use crate::components::{code::Code, preview::Preview};
use yew::prelude::*;

#[function_component(Block)]
pub fn block() -> Html {
    let code_1 = r#"html! {
    <ybc::Block>
        {"This is Block!"}
    </ybc::Block>
}"#;
    let preview_1 = html! {
        <ybc::Block>
            {"This is Block!"}
        </ybc::Block>
    };
    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Block"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
        </ybc::Container>
    }
}
