use std::rc::Rc;

use crate::components::description;
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

    let descriptions = vec![description::Item {
        title: "Footer".to_owned(),
        id: "footer".to_owned(),
        code: code_1.to_owned(),
        html: Rc::new(preview_1),
    }];

    let api: Vec<description::Api> = vec![(
        "<ybc::Footer>",
        "https://docs.rs/ybc/latest/ybc/struct.FooterProps.html",
    )
        .into()];

    html! {
        <description::Description items={descriptions} {api} />
    }
}
