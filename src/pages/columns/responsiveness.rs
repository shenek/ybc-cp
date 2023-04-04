use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(ColumnsResponsiveness)]
pub fn columns_responsiveness() -> Html {
    let code_1 = r#"html! {
    <ybc::Columns classes={classes!("is-mobile")}>
        <ybc::Column>{"1"}</ybc::Column>
        <ybc::Column>{"2"}</ybc::Column>
        <ybc::Column>{"3"}</ybc::Column>
        <ybc::Column>{"4"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_1 = html! {
        <ybc::Columns classes={classes!("is-mobile")}>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"2"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"3"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"4"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let code_2 = r#"html! {
    <ybc::Columns classes={classes!("is-desktop")}>
        <ybc::Column>{"1"}</ybc::Column>
        <ybc::Column>{"2"}</ybc::Column>
        <ybc::Column>{"3"}</ybc::Column>
        <ybc::Column>{"4"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_2 = html! {
        <ybc::Columns classes={classes!("is-desktop")}>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"2"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"3"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"4"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let code_3 = r#"html! {
    <ybc::Columns classes={classes!("is-mobile")}>
        <ybc::Column classes={classes!("is-three-quarters-mobile", "is-two-thirds-tablet", "is-half-desktop", "is-one-third-widescreen", "is-one-quarter-fullhd")}>
            <code>{"is-three-quarters-mobile"}</code><br />
            <code>{"is-two-thirds-tablet"}</code><br />
            <code>{"is-half-desktop"}</code><br />
            <code>{"is-one-third-widescreen"}</code><br />
            <code>{"is-one-quarter-fullhd"}</code>
        </ybc::Column>
        <ybc::Column>{"2"}</ybc::Column>
        <ybc::Column>{"3"}</ybc::Column>
        <ybc::Column>{"4"}</ybc::Column>
        <ybc::Column>{"5"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_3 = html! {
        <ybc::Columns classes={classes!("is-mobile")}>
            <ybc::Column classes={classes!("is-three-quarters-mobile", "is-two-thirds-tablet", "is-half-desktop", "is-one-third-widescreen", "is-one-quarter-fullhd")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >
                    <code>{"is-three-quarters-mobile"}</code><br />
                    <code>{"is-two-thirds-tablet"}</code><br />
                    <code>{"is-half-desktop"}</code><br />
                    <code>{"is-one-third-widescreen"}</code><br />
                    <code>{"is-one-quarter-fullhd"}</code>
                </ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"2"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"3"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"4"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"5"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let descriptions = vec![
        description::Item {
            title: "Columns mobile".to_owned(),
            id: "columns-desktop".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Columns desktop".to_owned(),
            id: "columns-mobile".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Different column sizes per breakpoint".to_owned(),
            id: "columns-breakpoint".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
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
