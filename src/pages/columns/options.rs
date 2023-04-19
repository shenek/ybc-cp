use std::rc::Rc;
use yew::prelude::*;

use crate::components::description;

#[function_component(ColumnsOptions)]
pub fn columns_options() -> Html {
    let code_1 = r#"html! {
    <ybc::Columns vcentered={true}>
        <ybc::Column classes={classes!("is-8")}>{"First Column"}</ybc::Column>
        <ybc::Column>
            {"Second Column with more content. This is so you can see the vertical alignment."}
        </ybc::Column>
    </ybc::Columns>
}"#;
    let preview_1 = html! {
        <ybc::Columns vcentered={true}>
            <ybc::Column classes={classes!("is-8")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"First Column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Second Column with more content. This is so you can see the vertical alignment."}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let code_2 = r#"html! {
    <ybc::Columns multiline={true} classes={classes!("is-mobile")}>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-half")}>{"is-half"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_2 = html! {
        <ybc::Columns multiline={true} classes={classes!("is-mobile")}>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-half")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-half"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let code_3 = r#"html! {
    <ybc::Columns centered={true} classes={classes!("is-mobile")}>
        <ybc::Column classes={classes!("is-half")}>{"is-half"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_3 = html! {
        <ybc::Columns centered={true} classes={classes!("is-mobile")}>
            <ybc::Column classes={classes!("is-half")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-half"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let code_4 = r#"html! {
    <ybc::Columns centered={true} multiline={true} classes={classes!("is-mobile")}>
        <ybc::Column classes={classes!("is-narrow")}>{"First Column"}</ybc::Column>
        <ybc::Column classes={classes!("is-narrow")}>{"Our Second Column"}</ybc::Column>
        <ybc::Column classes={classes!("is-narrow")}>{"Third Column"}</ybc::Column>
        <ybc::Column classes={classes!("is-narrow")}>{"The Fourth Column"}</ybc::Column>
        <ybc::Column classes={classes!("is-narrow")}>{"Fifth Column"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_4 = html! {
        <ybc::Columns centered={true} multiline={true} classes={classes!("is-mobile")}>
            <ybc::Column classes={classes!("is-narrow")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"First Column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-narrow")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Our Second column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-narrow")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Third Column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-narrow")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"The Fourth Column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-narrow")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Fifth Column"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let descriptions = vec![
        description::Item {
            title: "Vertical alignment".to_owned(),
            id: "columns-vertical-align".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Multiline".to_owned(),
            id: "columns-multiline".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Centering columns".to_owned(),
            id: "columns-centering1".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
        description::Item {
            title: "".to_owned(),
            id: "columns-centering2".to_owned(),
            code: code_4.to_owned(),
            html: Rc::new(preview_4),
        },
    ];

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
