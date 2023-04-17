use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(ColumnsNesting)]
pub fn columns_basics() -> Html {
    let code_1 = r#"html! {
    <ybc::Columns>
        <ybc::Column>
            {"First Column"}
            <ybc::Columns classes={classes!("is-mobile")}>
                <ybc::Column>{"First Nested Column"}</ybc::Column>
                <ybc::Column>{"Second Nested Column"}</ybc::Column>
            </ybc::Columns>
        </ybc::Column>
        <ybc::Column>
            {"Second Column"}
            <ybc::Columns classes={classes!("is-mobile")}>
                <ybc::Column classes={classes!("is-half")}>{"50%"}</ybc::Column>
                <ybc::Column>{"Auto"}</ybc::Column>
                <ybc::Column>{"Auto"}</ybc::Column>
            </ybc::Columns>
        </ybc::Column>
    </ybc::Columns>
}"#;
    let preview_1 = html! {
        <ybc::Columns>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-info", "has-text-centered", "has-text-weight-bold")}
                >{"First Column"}</ybc::Notification>
                <ybc::Columns classes={classes!("is-mobile")}>
                    <ybc::Column>
                        <ybc::Notification
                          classes={classes!("is-info", "has-text-centered", "has-text-weight-bold")}
                        >{"First Nested Column"}</ybc::Notification>
                    </ybc::Column>
                    <ybc::Column>
                        <ybc::Notification
                          classes={classes!("is-info", "has-text-centered", "has-text-weight-bold")}
                        >{"Second Nested Column"}</ybc::Notification>
                    </ybc::Column>
                </ybc::Columns>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-danger", "has-text-centered", "has-text-weight-bold")}
                >{"Second Column"}</ybc::Notification>
                <ybc::Columns classes={classes!("is-mobile")}>
                    <ybc::Column classes={classes!("is-half")}>
                        <ybc::Notification
                          classes={classes!("is-danger", "has-text-centered", "has-text-weight-bold")}
                        >{"50%"}</ybc::Notification>
                    </ybc::Column>
                    <ybc::Column>
                        <ybc::Notification
                          classes={classes!("is-danger", "has-text-centered", "has-text-weight-bold")}
                        >{"Auto"}</ybc::Notification>
                    </ybc::Column>
                    <ybc::Column>
                        <ybc::Notification
                          classes={classes!("is-danger", "has-text-centered", "has-text-weight-bold")}
                        >{"Auto"}</ybc::Notification>
                    </ybc::Column>
                </ybc::Columns>
            </ybc::Column>
        </ybc::Columns>
    };

    let descriptions = vec![description::Item {
        title: "Columns nesting".to_owned(),
        id: "columns-nesting".to_owned(),
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
