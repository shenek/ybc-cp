use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(ColumnsSizes)]
pub fn columns_sizes() -> Html {
    let code_1 = r#"html! {
    <ybc::Columns>
        <ybc::Column classes={classes!("is-full")}>{"is-full"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-four-fifths")}>{"is-four-fifths"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-three-quarters")}>{"is-three-quarters"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-two-thirds")}>{"is-two-thirds"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-three-fifths")}>{"is-three-fifths"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-half")}>{"is-half"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-two-fifths")}>{"is-two-fifths"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-one-third")}>{"is-one-third"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-one-fifth")}>{"is-one-fifth"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_1 = html! {
        <>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-full")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-full"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-four-fifths")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-four-fifths"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-three-quarters")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-three-quarters"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-two-thirds")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-two-thirds"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-three-fifths")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-three-fifths"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-half")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-half"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-two-fifths")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-two-fifths"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-one-third")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-third"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-one-fifth")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-fifth"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        </>
    };

    let code_2 = r#"html! {
    <ybc::Columns>
        <ybc::Column classes={classes!("is-1")}>{"is-1"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-2")}>{"is-2"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-3")}>{"is-3"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-4")}>{"is-4"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-5")}>{"is-5"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-6")}>{"is-6"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-7")}>{"is-7"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-8")}>{"is-8"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-9")}>{"is-9"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-10")}>{"is-10"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-11")}>{"is-11"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-12")}>{"is-12"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-2")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-2"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-3")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-3"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-4")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-4"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-5")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-5"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-6")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-6"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-7")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-7"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-8")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-8"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-9")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-9"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-10")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-10"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-11")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-11"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-1")}>
                <ybc::Notification
                  classes={classes!("has-text-centered", "has-text-weight-bold")}
                >{"1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-12")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-12"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        </>
    };

    let code_3 = r#"html! {
    <ybc::Columns>
        <ybc::Column classes={classes!("is-half", "is-offset-one-quarter")}>
            {"is-half"}<br />{"is-offset-one-quarter"}
        </ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-three-fifths", "is-offset-one-fifth")}>
            {"is-three-fifths"}<br />{"is-offset-one-fifth"}
        </ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-4", "is-offset-8")}>
            {"is-4"}<br />{"is-offset-8"}
        </ybc::Column>
    </ybc::Columns>
    <ybc::Columns>
        <ybc::Column classes={classes!("is-11", "is-offset-1")}>
            {"is-11"}<br />{"is-offset-1"}
        </ybc::Column>
    </ybc::Columns>
}"#;
    let preview_3 = html! {
        <>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-half", "is-offset-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-half"}<br />{"is-offset-one-quarter"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-three-fifths", "is-offset-one-fifth")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-three-fifths"}<br />{"is-offset-one-fifth"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-4", "is-offset-8")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-4"}<br />{"is-offset-8"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-11", "is-offset-1")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-11"}<br />{"is-offset-1"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        </>
    };

    let code_4 = r#"html! {
    <ybc::Columns>
        <ybc::Column classes={classes!("is-narrow")}>
            <div style="width: 200px;">
                <ybc::Box>
                    <ybc::Title
                      tag={"p"}
                      classes={classes!("is-5")}
                    >{"Narrow column"}</ybc::Title>
                    <ybc::Subtitle
                      tag={"p"}
                    >{"This column is only 200px wide."}</ybc::Subtitle>
                </ybc::Box>
            </div>
        </ybc::Column>
        <ybc::Column>
            <ybc::Box>
                <ybc::Title
                  tag={"p"}
                  classes={classes!("is-5")}
                >{"Flexible column"}</ybc::Title>
                <ybc::Subtitle
                  tag={"p"}
                >{"This column will take up the remaining space available."}</ybc::Subtitle>
            </ybc::Box>
        </ybc::Column>
    </ybc::Columns>
}"#;
    let preview_4 = html! {
        <ybc::Columns>
            <ybc::Column classes={classes!("is-narrow")}>
                <div style="width: 200px;">
                    <ybc::Box>
                        <ybc::Title
                          tag={"p"}
                          classes={classes!("is-5")}
                        >{"Narrow column"}</ybc::Title>
                        <ybc::Subtitle
                          tag={"p"}
                        >{"This column is only 200px wide."}</ybc::Subtitle>
                    </ybc::Box>
                </div>
            </ybc::Column>
            <ybc::Column>
                <ybc::Box>
                    <ybc::Title
                      tag={"p"}
                      classes={classes!("is-5")}
                    >{"Flexible column"}</ybc::Title>
                    <ybc::Subtitle
                      tag={"p"}
                    >{"This column will take up the remaining space available."}</ybc::Subtitle>
                </ybc::Box>
            </ybc::Column>
        </ybc::Columns>
    };

    let descriptions = vec![
        description::Item {
            title: "Column sizes".to_owned(),
            id: "columns-sizes".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "12 column system".to_owned(),
            id: "columns-12".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Offset".to_owned(),
            id: "columns-offset".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
        description::Item {
            title: "Narrow column".to_owned(),
            id: "columns-narrow".to_owned(),
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
