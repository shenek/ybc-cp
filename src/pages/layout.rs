use yew::prelude::*;

use crate::components::{code::Code, preview::Preview};

#[function_component(Container)]
pub fn container() -> Html {
    let code_1 = r#"html! {
    <ybc::Container>
        {"This is Container!"}
    </ybc::Container>
}"#;
    let preview_1 = html! {
        <ybc::Container>
            {"This is Container!"}
        </ybc::Container>

    };

    let code_2 = r#"html! {
    <ybc::Container fluid={true}>
        {"This is Container!"}
    </ybc::Container>
}"#;
    let preview_2 = html! {
        <ybc::Container fluid={true}>
            {"This is fluid Container! (extra padding)"}
        </ybc::Container>

    };

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Container"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Container Fluid"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
        </ybc::Container>
    }
}
