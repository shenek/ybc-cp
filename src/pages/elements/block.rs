use std::rc::Rc;

use crate::components::description;
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

    let descriptions = vec![description::Item {
        title: "Block".to_owned(),
        id: "block".to_owned(),
        code: code_1.to_owned(),
        html: Rc::new(preview_1),
    }];

    let api: Vec<description::Api> = vec![(
        "<ybc::Block>",
        "https://docs.rs/ybc/latest/ybc/struct.BlockProps.html",
    )
        .into()];
    html! {
        <description::Description items={descriptions} {api} />
    }
}
