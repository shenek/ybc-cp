use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(ColumnsBasics)]
pub fn columns_basics() -> Html {
    let code_1 = r#"html! {
    <ybc::Columns>
        <ybc::Column>{"First Column"}</ybc::Column>
        <ybc::Column>{"Second Column"}</ybc::Column>
        <ybc::Column>{"Third Column"}</ybc::Column>
        <ybc::Column>{"Fourth Column"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_1 = html! {
        <ybc::Columns>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"First Column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Second Column"}</ybc::Notification></ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Third Column"}</ybc::Notification></ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
            >{"Fourth Column"}</ybc::Notification></ybc::Column>
        </ybc::Columns>
    };

    let descriptions = vec![description::Item {
        title: "Columns basics".to_owned(),
        id: "columns".to_owned(),
        code: code_1.to_owned(),
        html: Rc::new(preview_1),
    }];

    let api: Vec<description::Api> = vec![
        (
            "<ybc::Column>",
            "https://docs.rs/ybc/latest/ybc/struct.ColumnProps.html",
        )
            .into(),
        (
            "<ybc::Columns>",
            "https://docs.rs/ybc/latest/ybc/struct.ColumnsProps.html",
        )
            .into(),
    ];
    html! {
        <description::Description items={descriptions} {api} />
    }
}
