use std::rc::Rc;

use yew::prelude::*;

use crate::components::description;

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

    let descriptions = vec![
        description::Item {
            title: "Container".to_owned(),
            id: "container".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Container Fluid".to_owned(),
            id: "container-fluid".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
    ];

    html! {
        <description::Description items={descriptions} />
    }
}
